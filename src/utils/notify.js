import {
  isPermissionGranted,
  requestPermission,
  sendNotification,
} from '@tauri-apps/api/notification'

export const notify = async (message) => {
  if (message == null) return

  let permissionGranted = await isPermissionGranted()

  // Get permission
  if (!permissionGranted) {
    const permission = await requestPermission()
    permissionGranted = permission === 'granted'
  }

  // Send notification
  if (permissionGranted) sendNotification(message)
}
