const dayjs = require('dayjs')
const formatCommit = require('./formatCommit')

/**
 * Format Log
 * @param {string} log - The line content from log
 * @param {boolean} isEN - Check whether the language is English.
 * @returns An object about log
 */
module.exports = function ({ log, isEN }) {
  const arr = String(log).split('|||')
  return {
    repo: arr[0] || '',
    author: arr[1] || '',
    email: arr[2] || '',
    commit: arr[3] || '',
    ...formatCommit({
      commit: arr[3],
      isEN,
    }),
    hash: arr[4].replace(/'/, '#') || '',
    time: dayjs(arr[5]).format('YYYY-MM-DD HH:mm:ss'),
    unix: dayjs(arr[5]).unix() * 1000,
  }
}
