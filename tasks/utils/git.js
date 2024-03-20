import { exec } from 'node:child_process'

/**
 * Executes a command and returns a promise.
 *
 * @param {string} command - The command to be executed.
 * @return {Promise} A promise that resolves with the output of the command.
 */
const executeCommand = async (command) => {
  return new Promise((resolve, reject) => {
    exec(command, (error, stdout, stderr) => {
      if (error) {
        reject(error)
        return
      }
      if (stderr) {
        reject(stderr)
        return
      }
      resolve(stdout)
    })
  })
}

const isCurrentBranch = async (branch) => {
  try {
    const getBranch = await executeCommand('git branch --show-current')
    const currentBranch = getBranch.toString().trim().toLowerCase()
    if (currentBranch === branch.toLowerCase()) return true
    else return false
  } catch (error) {
    console.error('Error checking current branch:', error)
    return false
  }
}

/**
 * Check if the current Git branch is clean.
 *
 * @return {boolean} true if the branch is clean, false otherwise
 */
const isCurrentBranchClean = async () => {
  try {
    const status = await executeCommand('git status --porcelain')
    return status.length === 0
  } catch (error) {
    console.error('Error checking if branch is clean:', error)
    process.exit(0)
  }
}

/**
 * Creates a new branch from the main branch and checks it out.
 *
 * @param {string} branchName - The name of the new branch to create.
 * @param {string} fromBranch - The name of the branch to create the new branch from.
 * @return {Promise<void>} - A Promise that resolves when the new branch is created and checked out.
 */
export const createBranch = async (branchName, fromBranch = 'main') => {
  try {
    // Check if branch is clean
    const isBranchClean = await isCurrentBranchClean()
    if (!isBranchClean) return false

    // Checkout main branch if not already
    const isBranchMain = await isCurrentBranch('main')
    if (!isBranchMain) await executeCommand(`git checkout ${fromBranch}`)

    // Create and check out to the new branch
    await executeCommand(`git checkout -b ${branchName}`)

    return true
  } catch (error) {
    return false
  }
}
