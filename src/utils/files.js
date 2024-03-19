import { formatNumber } from '@src/utils/numbers'

/**
 * Returns a formatted file size in KB, MB, or GB based on the input bytes and decimal precision.
 *
 * @param {number} bytes - The size of the file in bytes
 * @param {number} [decimals=0] - The number of decimal places for precision
 * @return {string} The formatted file size with appropriate unit (KB, MB, or GB)
 */
export const fileSize = (bytes, decimals = 0) => {
  const kb = bytes / 1000
  const mb = bytes / 1000 / 1000
  const gb = bytes / 1000 / 1000 / 1000

  if (gb >= 1) return `${formatNumber(gb.toFixed(decimals))} GB`
  else if (mb >= 1) return `${formatNumber(mb.toFixed(decimals))} MB`
  else return `${formatNumber(kb.toFixed(decimals))} KB`
}

/**
 * Determines the type of file based on its extension.
 *
 * @param {Object} item - the file metadata
 * @return {Object} - an object indicating the type of the file
 */
export const fileType = (item) => {
  // Exit if not a file
  if (!item.metadata.is_file) return

  // Determine extension and set results
  const extension = item.name.split('.').pop().toLowerCase()
  const result = { audio: false, image: false, other: false, video: false }

  // File type arrays
  const audios = ['mp3', 'wav', 'ogg', 'flac', 'm4a', 'wma', 'aiff']
  const images = ['jpg', 'jpeg', 'png', 'gif', 'webp', 'svg', 'pjpeg']
  const videos = ['mp4', 'mkv', 'avi', 'mov', 'webm', 'mpg', 'mpeg', 'm4v']

  // Set type true and return
  if (audios.includes(extension)) {
    result.audio = true
    return result
  }
  if (images.includes(extension)) {
    result.image = true
    return result
  }
  if (videos.includes(extension)) {
    result.video = true
    return result
  }

  // Default to other
  result.other = true
  return result
}
