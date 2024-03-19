// Utilities
import { $settings } from '@src/utils/settings'

/**
 * ignoredFiles
 * @type {Array}
 */
export const defaultIgnores = [
  '.DS_Store',
  '.git',
  '.github',
  '.nuxt',
  '.pnp',
  '.yarn',
  'debug',
  'node_modules',
  'src-tauri/target',
]

/**
 * ignoreList
 * @type {Array}
 */
export const ignoreList = async () => {
  // Get saved items and standard ignored items
  const savedItems = await $settings.get('ignore')
  let ignoredItems = defaultIgnores

  // Get user saved items
  if (savedItems != null && savedItems.value.length > 0) {
    const savedArray = savedItems.value.split(',')
    ignoredItems = ignoredItems.concat(savedArray)
  }

  // Clean filtered items incase bad array items
  ignoredItems = ignoredItems
    .map((item) => item.trim().toLowerCase())
    .filter((item) => item.length > 0 && item != null)

  // Return unique array
  return [...new Set(ignoredItems)]
}

export const findDir = ({ parentPath, paths, projects, type }) => {
  const result = {
    type: {
      access: type.hasOwnProperty('access'),
      create: type.hasOwnProperty('create'),
      modify: type.hasOwnProperty('modify'),
      remove: type.hasOwnProperty('remove'),
    },
  }

  const parentDir = (dir) =>
    dir
      .split('/')
      .slice(0, -1)
      .join('/')
      .replace(parentPath, '')
      .replace('/', '')

  const findDir = (path) => {
    const prop = projects.find((p) => p.name === parentDir(path))
    return prop == null ? projects : prop
  }

  const pathsList = paths.map((path) => {
    return {
      parent: parentDir(path),
      property: findDir(path),
    }
  })

  return Object.assign(result, { paths: pathsList })
}
