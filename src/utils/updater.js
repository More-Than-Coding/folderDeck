import { checkUpdate, installUpdate } from '@tauri-apps/api/updater'
import { relaunch } from '@tauri-apps/api/process'

export const updateApp = async () => {
  try {
    // Set variables
    const { shouldUpdate, manifest } = await checkUpdate()

    // Early exist if offline or should not update
    if (!window.navigator.onLine || !shouldUpdate) return

    // Install update and relaunch
    await installUpdate()
    await relaunch()
  } catch (error) {
    console.error(error)
  }
}
