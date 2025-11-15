import { ref, onMounted } from 'vue'
import { check } from '@tauri-apps/plugin-updater'
import { ask } from '@tauri-apps/plugin-dialog'

export function useUpdater() {
  const updateAvailable = ref(false)
  const updateVersion = ref('')
  const checking = ref(false)
  const downloading = ref(false)
  const downloadProgress = ref(0)

  async function checkForUpdates(silent = false) {
    if (checking.value) return

    checking.value = true

    try {
      const update = await check()

      if (update) {
        updateAvailable.value = true
        updateVersion.value = update.version

        if (!silent) {
          const yes = await ask(
            `Update to version ${update.version} is available!\n\nRelease notes:\n${update.body || 'No release notes provided'}\n\nWould you like to install it now?`,
            {
              title: 'Update Available',
              kind: 'info',
            }
          )

          if (yes) {
            await downloadAndInstall(update)
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

  async function downloadAndInstall(update: any) {
    downloading.value = true
    downloadProgress.value = 0

    try {
      // Download and install the update
      await update.downloadAndInstall((event: any) => {
        switch (event.event) {
          case 'Started':
            downloadProgress.value = 0
            break
          case 'Progress':
            downloadProgress.value = event.data.chunkLength
            break
          case 'Finished':
            downloadProgress.value = 100
            break
        }
      })

      // App will restart automatically after update installs
      await ask('Update installed successfully! The app will restart now.', {
        title: 'Update Complete',
        kind: 'info',
      })
    } catch (error) {
      console.error('Failed to install update:', error)
      await ask(`Failed to install update: ${error}`, {
        title: 'Update Error',
        kind: 'error',
      })
    } finally {
      downloading.value = false
    }
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
    checking,
    downloading,
    downloadProgress,
    checkForUpdates,
  }
}
