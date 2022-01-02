const dayjs = require('dayjs')
const formatCommit = require('./formatCommit')

module.exports = function (log) {
  const arr = String(log).split('|||')
  return {
    repo: arr[0] || '',
    author: arr[1] || '',
    email: arr[2] || '',
    commit: arr[3] || '',
    ...formatCommit(arr[3]),
    hash: arr[4].replace(/'/, '#') || '',
    time: dayjs(arr[5]).format('YYYY-MM-DD HH:mm:ss'),
    unix: dayjs(arr[5]).unix() * 1000,
  }
}
