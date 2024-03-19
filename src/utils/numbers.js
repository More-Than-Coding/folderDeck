/**
 * Formats a number using the Intl.NumberFormat object.
 *
 * @param {number} number - the number to be formatted
 * @return {string} the formatted number as a string
 */
export const formatNumber = (number) => {
  return new Intl.NumberFormat('en-US', {
    style: 'decimal',
    maximumFractionDigits: 2,
  }).format(parseFloat(number))
}

/**
 * Formats a number into a currency string.
 *
 * @param {number} number - The number to be formatted as currency
 * @return {string} The formatted currency string
 */
export const formatCurrency = (number) => {
  return new Intl.NumberFormat('en-US', {
    style: 'currency',
    currency: 'USD',
  }).format(parseFloat(number))
}
