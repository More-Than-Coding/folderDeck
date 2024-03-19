import { trackEvent } from '@aptabase/tauri'
import { $settings } from '@src/utils/settings'

/**
 * Track - Tracking events
 *
 * @param {String} name - name of the event
 * @param {Object} obj - data to track
 */
export const track = async (name, obj = null) => {
  try {
    // Check if tracking is enabled
    const trackingEnabled = await $settings.get('analytics')
    if (trackingEnabled != null && !trackingEnabled.value) return

    // Exist if offline
    if (!window.navigator.onLine) return

    // Track with/without object
    if (obj == null) trackEvent(name)
    else trackEvent(name, obj)
  } catch (error) {
    // Log error
    console.error('Error', error)
  }
}
