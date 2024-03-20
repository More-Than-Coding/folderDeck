export const devLog = ({ title, message, error = null }) => {
  // Do not log in production
  if (import.meta.env.PROD) return

  // Setup styling for basic output
  let css = 'color: #FFCB6B; background: #2B2B2B; padding: 4px 2px;'

  if (error != null) {
    css = 'color: #2B2B2B; background: #fca5a5; padding: 4px 2px;'
  }

  // Determine output of item
  switch (typeof message) {
    case 'object':
      console.debug(
        `\%c ${title.trim()}: ${Object.keys(message).length} keys `,
        css
      )
      console.debug(message)
      break

    case 'array':
      console.debug(`\%c ${title.trim()}: ${message.length} items `, css)
      console.debug(message)
      break

    default:
      console.debug(`\%c ${title.trim()}: ${message.trim()} `, css)
      break
  }

  if (error != null) {
    console.error(error)
  }
}
