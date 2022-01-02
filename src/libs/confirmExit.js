/**
 * 退出确认
 */
module.exports = function (msg) {
  console.log('')
  console.log(msg)
  console.log(`按下任意键退出…`)
  process.stdin.setRawMode(true)
  process.stdin.resume()
  process.stdin.on('data', process.exit.bind(process, 0))
}
