/**
 * Calculate the memory size of an object in bytes and format the size in KB, MB, or GB.
 *
 * @param {object} object - The object to calculate the memory size of
 * @return {string} The formatted memory size
 */
export const memorySizeOf = (object) => {
  let size = 0

  const calculateSize = (item) => {
    switch (typeof item) {
      case 'number':
        size += 8
        break
      case 'string':
        size += item.length * 2
        break
      case 'boolean':
        size += 4
        break
      case 'object':
        if (item === null) break
        const itemClass = Object.prototype.toString.call(item).slice(8, -1)
        if (itemClass === 'Object' || itemClass === 'Array') {
          for (const key in item) {
            if (item.hasOwnProperty(key)) calculateSize(item[key])
          }
        } else {
          size += item.toString().length * 2
        }
        break
    }
  }

  calculateSize(object)

  return formatSize(size)
}

/**
 * Convert a given time in milliseconds to a human-readable format.
 *
 * @param {number} time - The time in milliseconds to be converted
 * @return {string} The time converted to a human-readable format
 */
export const timeConvertMs = (time) => {
  if (time > 1000 * 60) return `${(time / 1000 / 60).toFixed(1)} m`
  if (time > 1000) return `${(time / 1000).toFixed(1)} s`
  return `${parseInt(time)} ms`
}
