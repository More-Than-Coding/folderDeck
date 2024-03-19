export const pathFilename = (path, lowerCase = false) => {
  if (lowerCase) return path.split('/').pop().toLowerCase()
  return path.split('/').pop()
}
