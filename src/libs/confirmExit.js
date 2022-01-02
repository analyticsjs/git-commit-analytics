/**
 * Confirm Exit
 * @param {string} msg - The message to displayed on console.
 * @param {boolean} isEN - Check whether the language is English.
 */
module.exports = function ({ msg, isEN }) {
  console.log('')
  console.log(msg)
  console.log(isEN ? `Press any key to exit…` : `按下任意键退出…`)
  process.stdin.setRawMode(true)
  process.stdin.resume()
  process.stdin.on('data', process.exit.bind(process, 0))
}
