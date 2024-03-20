import { checkUpdate, installUpdate } from '@tauri-apps/api/updater'
import { relaunch } from '@tauri-apps/api/process'
import { devLog } from '@src/utils/log'

/**
 * Check for update asynchronously
 *
 * @return {boolean} indicates whether an update is available
 */
export const checkForUpdate = async () => {
  try {
    // Do not attempt if offline
    if (!window.navigator.onLine) return false

    // Check for update
    const { shouldUpdate } = await checkUpdate()
    return shouldUpdate
  } catch (error) {
    devLog({
      title: '🚨 Updater Error',
      message: `Checking for release failed. "${error}"`,
      error: true,
    })
    return
  }
}

export const updateApp = async () => {
  try {
    // Early exist if offline or should not update
    if (!window.navigator.onLine) return

    // const install = await installUpdate()
    // console.log('install', install)
    // await relaunch()
    return
  } catch (error) {
    devLog({
      title: '🚨 Updater Error',
      message: `Unable to check for release update. "${error}"`,
      error: true,
    })
    return
  }
}
