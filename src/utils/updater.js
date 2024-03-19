import { checkUpdate, installUpdate } from '@tauri-apps/api/updater'
import { relaunch } from '@tauri-apps/api/process'
import { devLog } from '@src/utils/log'

export const updateApp = async () => {
  try {
    // Set variables
    const { shouldUpdate } = await checkUpdate()

    console.log('shouldUpdate', shouldUpdate)

    // Early exist if offline or should not update
    if (!window.navigator.onLine || !shouldUpdate) return

    // Install update and relaunch
    await installUpdate()
    await relaunch()
  } catch (error) {
    devLog({
      title: '🚨 Updater Error',
      message: `Unable to check for release update. "${error}"`,
      error: true,
    })
  }
}
