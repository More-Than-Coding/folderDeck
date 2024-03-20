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
