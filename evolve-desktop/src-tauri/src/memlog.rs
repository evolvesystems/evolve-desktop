//! Memory numbers in evolveapp.log, once a minute (task 5469): this app's resident memory and the WebView2 processes, so a "memory hungry" report can be measured.
//! std-only parsing and formatting, tested with plain `rustc --test src/memlog.rs`. Collection shells out (tasklist on Windows, ps elsewhere) so no new dependency is added;
//! WebView2 is shared by other apps on the machine, so the webview figures are for ALL msedgewebview2.exe processes, not only this app's (the log line says so).

/// Seconds between samples.
pub const EVERY_SECS: u64 = 60;

/// "54,321 K" or "54.321 KB" or "54 321 K" -> 54321. Only digits are kept, so the thousands separator of any language works.
pub fn parse_kb(field: &str) -> Option<u64> {
    let digits: String = field.chars().filter(|c| c.is_ascii_digit()).collect();
    if digits.is_empty() || digits.len() > 12 {
        return None;
    }
    digits.parse().ok()
}

/// Splits one tasklist /FO CSV /NH line into its fields: "name","pid","session","num","54,321 K".
pub fn csv_fields(line: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut quoted = false;
    for c in line.trim().chars() {
        match c {
            '"' => quoted = !quoted,
            ',' if !quoted => out.push(std::mem::take(&mut cur)),
            _ => cur.push(c),
        }
    }
    out.push(cur);
    out
}

/// (process count, total KB) from tasklist CSV output; lines that are not a process row (the "no tasks" notice) are ignored.
pub fn sum_tasklist(output: &str) -> (usize, u64) {
    let mut n = 0;
    let mut kb = 0u64;
    for line in output.lines() {
        let f = csv_fields(line);
        if f.len() >= 5 && f[1].chars().all(|c| c.is_ascii_digit()) && !f[1].is_empty() {
            if let Some(k) = parse_kb(&f[4]) {
                n += 1;
                kb = kb.saturating_add(k);
            }
        }
    }
    (n, kb)
}

/// "VmRSS:   123456 kB" from /proc/self/status.
pub fn rss_from_proc_status(text: &str) -> Option<u64> {
    text.lines().find(|l| l.starts_with("VmRSS:")).and_then(|l| parse_kb(l.trim_start_matches("VmRSS:")))
}

pub fn log_line(app_kb: Option<u64>, webview: Option<(usize, u64)>, parked: bool, hidden: bool) -> String {
    let mb = |kb: u64| kb / 1024;
    let app = app_kb.map(|k| format!("{} MB", mb(k))).unwrap_or_else(|| "?".to_string());
    let wv = webview.map(|(n, k)| format!("{} procs {} MB (all apps)", n, mb(k))).unwrap_or_else(|| "?".to_string());
    format!("mem app={} webview2={} window={} content={}", app, wv, if hidden { "hidden" } else { "shown" }, if parked { "parked" } else { "loaded" })
}

/// Runs a command without flashing a console window; None if it cannot run or fails.
fn run(cmd: &str, args: &[&str]) -> Option<String> {
    let mut c = std::process::Command::new(cmd);
    c.args(args);
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        c.creation_flags(0x0800_0000); // CREATE_NO_WINDOW
    }
    let out = c.output().ok()?;
    if !out.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&out.stdout).into_owned())
}

/// This process's resident memory in KB.
pub fn app_rss_kb() -> Option<u64> {
    let pid = std::process::id().to_string();
    #[cfg(target_os = "linux")]
    {
        let _ = &pid;
        return rss_from_proc_status(&std::fs::read_to_string("/proc/self/status").ok()?);
    }
    #[cfg(windows)]
    {
        let out = run("tasklist", &["/FI", &format!("PID eq {pid}"), "/FO", "CSV", "/NH"])?;
        let (n, kb) = sum_tasklist(&out);
        return if n == 1 { Some(kb) } else { None };
    }
    #[cfg(not(any(target_os = "linux", windows)))]
    {
        return parse_kb(&run("ps", &["-o", "rss=", "-p", &pid])?);
    }
}

/// (count, total KB) of every msedgewebview2.exe on the machine (Windows only; other platforms have no such process).
pub fn webview_procs() -> Option<(usize, u64)> {
    #[cfg(windows)]
    {
        return Some(sum_tasklist(&run("tasklist", &["/FI", "IMAGENAME eq msedgewebview2.exe", "/FO", "CSV", "/NH"])?));
    }
    #[cfg(not(windows))]
    {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_reads_memory_with_any_thousands_separator() {
        assert_eq!(parse_kb("54,321 K"), Some(54321));
        assert_eq!(parse_kb("54.321 KB"), Some(54321));
        assert_eq!(parse_kb("54\u{a0}321 K"), Some(54321));
        assert_eq!(parse_kb("N/A"), None);
        assert_eq!(parse_kb(""), None);
        assert_eq!(parse_kb("99999999999999 K"), None);
    }

    #[test]
    fn it_sums_a_tasklist_listing() {
        let out = "\"msedgewebview2.exe\",\"1204\",\"Console\",\"1\",\"120,500 K\"\r\n\"msedgewebview2.exe\",\"2210\",\"Console\",\"1\",\"80,000 K\"\r\n\"msedgewebview2.exe\",\"3318\",\"Console\",\"1\",\"9,500 K\"\r\n";
        assert_eq!(sum_tasklist(out), (3, 210_000));
    }

    #[test]
    fn the_no_tasks_notice_and_blank_lines_count_for_nothing() {
        assert_eq!(sum_tasklist("INFO: No tasks are running which match the specified criteria.\r\n\r\n"), (0, 0));
        assert_eq!(sum_tasklist(""), (0, 0));
    }

    #[test]
    fn a_comma_inside_quotes_does_not_split_a_field() {
        assert_eq!(csv_fields("\"a.exe\",\"12\",\"Console\",\"1\",\"1,234 K\""), vec!["a.exe", "12", "Console", "1", "1,234 K"]);
    }

    #[test]
    fn it_reads_rss_from_proc_status() {
        assert_eq!(rss_from_proc_status("Name:\tx\nVmRSS:\t  123456 kB\nThreads:\t4\n"), Some(123456));
        assert_eq!(rss_from_proc_status("Name:\tx\n"), None);
    }

    #[test]
    fn the_log_line_has_the_numbers_the_state_and_no_paths() {
        let l = log_line(Some(204_800), Some((7, 1_048_576)), true, true);
        assert_eq!(l, "mem app=200 MB webview2=7 procs 1024 MB (all apps) window=hidden content=parked");
        assert_eq!(log_line(None, None, false, false), "mem app=? webview2=? window=shown content=loaded");
    }
}
