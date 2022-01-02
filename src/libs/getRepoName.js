const { resolve, sep } = require('path')

/**
 * Get Repo Name
 * @param {*} repoDir - The item in configuration `config.repos`
 * @param {*} format - The configuration `config.format`
 * @returns The repository name
 */
module.exports = function (repoDir, format) {
  const arr = resolve(repoDir).split(sep)
  if (!arr.length) {
    return 'undefined'
  }

  const i = arr.length - 1
  const key = arr[i]
  return format[key]
}
