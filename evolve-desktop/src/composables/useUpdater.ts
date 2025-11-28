import { ref, onMounted } from 'vue'
import { ask } from '@tauri-apps/plugin-dialog'
import { open } from '@tauri-apps/plugin-shell'
import { getVersion } from '@tauri-apps/api/app'
import { platform } from '@tauri-apps/plugin-os'

interface VersionInfo {
  version: string
  releaseNotes: string
  releaseDate: string
  downloadUrls: {
    linux: {
      appimage: string | null
      deb: string | null
      rpm: string | null
    }
    windows: string | null
    macos: string | null
  }
}

export function useUpdater() {
  const updateAvailable = ref(false)
  const updateVersion = ref('')
  const updateNotes = ref('')
  const downloadUrl = ref('')
  const checking = ref(false)

  async function checkForUpdates(silent = false) {
    if (checking.value) return

    checking.value = true

    try {
      // Get current app version
      const currentVersion = await getVersion()

      // Check EIQ Manager API for latest version
      const response = await fetch('https://evolvepreneuriq.app/api/v1/desktop/version')

      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`)
      }

      const data: VersionInfo = await response.json()

      // Compare versions
      if (compareVersions(data.version, currentVersion) > 0) {
        // Update available
        updateAvailable.value = true
        updateVersion.value = data.version
        updateNotes.value = data.releaseNotes

        // Get download URL for current platform
        const currentPlatform = await platform()
        downloadUrl.value = getDownloadUrl(currentPlatform, data.downloadUrls)

        if (!silent) {
          const yes = await ask(
            `Update to version ${data.version} is available!\n\nRelease notes:\n${data.releaseNotes}\n\nWould you like to download it now?`,
            {
              title: 'Update Available',
              kind: 'info',
            }
          )

          if (yes) {
            await openDownloadPage()
          }
        }
      } else if (!silent) {
        await ask('You are on the latest version!', {
          title: 'No Updates',
          kind: 'info',
        })
      }
    } catch (error) {
      if (!silent) {
        console.error('Failed to check for updates:', error)
        await ask(`Failed to check for updates: ${error}`, {
          title: 'Update Error',
          kind: 'error',
        })
      }
    } finally {
      checking.value = false
    }
  }

  async function openDownloadPage() {
    if (downloadUrl.value) {
      await open(downloadUrl.value)
    }
  }

  function getDownloadUrl(platformName: string, urls: VersionInfo['downloadUrls']): string {
    switch (platformName.toLowerCase()) {
      case 'linux':
        // Prefer AppImage, fallback to deb
        return urls.linux.appimage || urls.linux.deb || urls.linux.rpm || ''
      case 'windows':
        return urls.windows || ''
      case 'macos':
      case 'darwin':
        return urls.macos || ''
      default:
        return ''
    }
  }

  function compareVersions(v1: string, v2: string): number {
    const parts1 = v1.split('.').map(Number)
    const parts2 = v2.split('.').map(Number)

    for (let i = 0; i < Math.max(parts1.length, parts2.length); i++) {
      const part1 = parts1[i] || 0
      const part2 = parts2[i] || 0

      if (part1 > part2) return 1
      if (part1 < part2) return -1
    }

    return 0
  }

  // Check for updates on mount (silent check)
  onMounted(() => {
    // Check for updates 5 seconds after app starts
    setTimeout(() => {
      checkForUpdates(true)
    }, 5000)
  })

  return {
    updateAvailable,
    updateVersion,
    updateNotes,
    downloadUrl,
    checking,
    checkForUpdates,
    openDownloadPage,
  }
}
