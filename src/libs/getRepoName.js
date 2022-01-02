const { resolve, sep } = require('path')

module.exports = function (repoDir, format) {
  const arr = resolve(repoDir).split(sep)
  if (!arr.length) {
    return 'undefined'
  }

  const i = arr.length - 1
  const key = arr[i]
  return format[key]
}
