import fs from 'node:fs'
import { cwd } from './dir.js'

/**
 * Updates the version in the package.json file.
 *
 * @param {string} version - The new version to update to
 * @return {void}
 */
const updatePackage = ({ key, value }) => {
  const path = cwd('package.json')
  const file = fs.readFileSync(path, 'utf8')
  const data = JSON.parse(file)
  data[key] = value
  fs.writeFileSync(path, JSON.stringify(data, null, 2))
}

/**
 * Updates the version in the Cargo.toml file to the provided version.
 *
 * @param {string} version - The new version to update to
 * @return {void}
 */
const updateCargo = ({ key, value }) => {
  const path = cwd('src-tauri/Cargo.toml')
  const file = fs.readFileSync(path, 'utf8')
  const regex = new RegExp(`^${key}\\s*=\\s*"[^"]+"$`, 'gm')
  const update = file.replace(regex, `${key} = "${value}"`)
  fs.writeFileSync(path, update)
}

/**
 * Updates the Tauri configuration file with a new version.
 *
 * @param {string} version - The new version to update to
 * @return {void}
 */
const updateTauri = ({ key, value }) => {
  const path = cwd('src-tauri/tauri.conf.json')
  const file = fs.readFileSync(path, 'utf8')
  const data = JSON.parse(file)
  data.package[key] = value
  fs.writeFileSync(path, JSON.stringify(data, null, 2))
}

/**
 * Updates versions for all files including Cargo, Tauri, and Package.
 *
 * @param {type} version - The version to update to
 * @return {void}
 */
export const updateVersions = (version) => {
  updateCargo({ key: 'version', value: version })
  updateTauri({ key: 'version', value: version })
  updatePackage({ key: 'version', value: version })
}

/**
 * Retrieves the current version from the package.json file and returns the version number as a string and an array.
 *
 * @return {Object} An object containing the version number as a string and an array of version numbers.
 */
export const currentVersion = () => {
  const pkgFile = fs.readFileSync(cwd('package.json'), 'utf8')
  const version = JSON.parse(pkgFile).version
  return {
    version,
    versionArray: version.split('.').map((v) => parseInt(v)),
  }
}
