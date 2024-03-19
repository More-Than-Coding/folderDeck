export const devLog = ({ title, message }) => {
  // Do not log in production
  if (import.meta.env.PROD) return

  // Setup styling for basic output
  const css = 'color:#FFCB6B; background: #2B2B2B; padding: 2px 4px;'

  // Determine output of item
  switch (typeof message) {
    case 'object':
      console.debug(`\%c ${title}: ${Object.keys(message).length} keys `, css)
      console.debug(message)
      break

    case 'array':
      console.debug(`\%c ${title}: ${message.length} items `, css)
      console.debug(message)
      break

    default:
      console.debug(`\%c ${title}: ${message} `, css)
      break
  }
}
