#!/bin/bash

# GitHub API configuration
GITHUB_TOKEN="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"
REPO="evolvesystems/evolve-desktop"
RUN_ID="19390845594"

echo "🔍 Monitoring GitHub Actions build for evolve-desktop..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Function to get job status
get_status() {
    curl -s -H "Authorization: token $GITHUB_TOKEN" \
         -H "Accept: application/vnd.github+json" \
         "https://api.github.com/repos/$REPO/actions/runs/$RUN_ID/jobs"
}

# Function to check if Windows build is complete
check_windows_status() {
    local response="$1"
    echo "$response" | python3 -c "
import sys, json
data = json.load(sys.stdin)
jobs = data.get('jobs', [])

windows_job = None
for job in jobs:
    if 'windows-latest' in job.get('name', ''):
        windows_job = job
        break

if windows_job:
    status = windows_job.get('status')
    conclusion = windows_job.get('conclusion')
    name = windows_job.get('name')

    print(f'{status}|{conclusion}|{name}')
else:
    print('not_found||')
"
}

# Function to display current status
show_status() {
    local response="$1"
    echo "$response" | python3 -c "
import sys, json
from datetime import datetime, timezone

data = json.load(sys.stdin)
jobs = data.get('jobs', [])

print('Current Build Status:')
print('─' * 60)

for job in jobs:
    name = job.get('name', '')
    status = job.get('status', '')
    conclusion = job.get('conclusion', '')

    # Determine icon
    if conclusion == 'success':
        icon = '✅'
    elif conclusion == 'failure':
        icon = '❌'
    elif status == 'in_progress':
        icon = '⏳'
    elif status == 'queued':
        icon = '⏸️'
    else:
        icon = '○'

    # Extract platform
    platform = 'Unknown'
    if 'windows' in name.lower():
        platform = 'Windows'
    elif 'ubuntu' in name.lower() or 'linux' in name.lower():
        platform = 'Linux'
    elif 'macos' in name.lower():
        if 'aarch64' in name:
            platform = 'macOS (ARM64)'
        else:
            platform = 'macOS (Intel)'

    status_text = conclusion if conclusion else status
    print(f'{icon} {platform:20} {status_text:15}')

print('─' * 60)
"
}

# Main monitoring loop
LAST_STATUS=""
CHECK_INTERVAL=30  # Check every 30 seconds

while true; do
    # Get current status
    RESPONSE=$(get_status)

    # Parse Windows job status
    WIN_STATUS=$(check_windows_status "$RESPONSE")
    IFS='|' read -r STATUS CONCLUSION NAME <<< "$WIN_STATUS"

    # Display status if changed
    CURRENT_STATUS="$STATUS|$CONCLUSION"
    if [ "$CURRENT_STATUS" != "$LAST_STATUS" ]; then
        clear
        echo "🔍 Monitoring GitHub Actions build for evolve-desktop..."
        echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
        echo ""
        show_status "$RESPONSE"
        echo ""
        echo "Last updated: $(date '+%Y-%m-%d %H:%M:%S')"
        echo ""

        LAST_STATUS="$CURRENT_STATUS"
    fi

    # Check if Windows build is complete
    if [ "$STATUS" = "completed" ]; then
        echo ""
        echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

        if [ "$CONCLUSION" = "success" ]; then
            echo "🎉 Windows build completed successfully!"
            echo ""
            echo "You can download the Windows installer from:"
            echo "https://github.com/$REPO/actions/runs/$RUN_ID"
            echo ""
            echo "Or check the releases page:"
            echo "https://github.com/$REPO/releases"

            # Play a bell sound
            echo -e "\a"

        elif [ "$CONCLUSION" = "failure" ]; then
            echo "❌ Windows build failed!"
            echo ""
            echo "Check the logs at:"
            echo "https://github.com/$REPO/actions/runs/$RUN_ID"
        else
            echo "⚠️  Windows build completed with status: $CONCLUSION"
            echo ""
            echo "Check the details at:"
            echo "https://github.com/$REPO/actions/runs/$RUN_ID"
        fi

        echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
        break
    fi

    # Wait before next check
    sleep $CHECK_INTERVAL
done
