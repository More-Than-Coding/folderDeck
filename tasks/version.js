import {
  cancel,
  confirm,
  intro,
  isCancel,
  outro,
  select,
  text,
} from '@clack/prompts'
import { currentVersion, updateVersions } from './utils/update.js'
import { createBranch } from './utils/git.js'

// Variables
let version = null

// Cancellation
const cancelled = () => {
  cancel('No changes were made.')
  process.exit(0)
}

// START
intro('Update app version')

// Select Version
const selectVersion = await select({
  message: 'How do you want to update the version?',
  options: [
    { value: true, label: 'Update version' },
    { value: false, label: 'Set a custom version' },
  ],
})

// Cancelled
if (isCancel(selectVersion)) cancelled()

// Set version with select options
if (selectVersion) {
  const currentArray = currentVersion().versionArray
  const newVersions = {
    major: `${currentArray[0] + 1}.0.0`,
    minor: `${currentArray[0]}.${currentArray[1] + 1}.0`,
    patch: `${currentArray[0]}.${currentArray[1]}.${currentArray[2] + 1}`,
  }

  // Select Version
  version = await select({
    message: 'Which version update are you making?',
    options: [
      {
        value: newVersions.major,
        label: `Major (${newVersions.major})`,
        hint: 'Breaking changes or redesigns',
      },
      {
        value: newVersions.minor,
        label: `Minor (${newVersions.minor})`,
        hint: 'Adding new features or functionality',
      },
      {
        value: newVersions.patch,
        label: `Patch (${newVersions.patch})`,
        hint: 'Bug fixes or applying package updates',
      },
    ],
  })
}

// Set custom version manually
if (!selectVersion) {
  const current = currentVersion().version
  const regex = /^\d+\.\d+\.\d+(-[a-zA-Z0-9]+(\.[a-zA-Z0-9]+)?)?$/

  version = await text({
    message: 'Enter the version number (eg. X.X.X | X.X.X-beta | X.X.X-rc.1):',
    placeholder: current,
    initialValue: current,
    validate(value) {
      if (value === current) return 'Input is the same as the current version'
      if (regex.test(value) === false)
        return `Input does not match the format X.X.X`
    },
  })
}

// Create a release branch
const createRelease = await confirm({
  message: 'Do you want to create a branch (this will branch from `main`)?',
})

if (createRelease) {
  const branchCreated = await createBranch(`v${version}`)

  // Exit if could not create release branch
  if (!branchCreated) {
    cancel('There branch is not clean or this version branch already exists.')
    process.exit(0)
  }
}

// If selections are cancelled
if (isCancel(version) || isCancel(createRelease)) cancelled()

// Update version
updateVersions(version)

// END
outro(`The app was updated to v${version}`)
