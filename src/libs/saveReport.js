const { writeFileSync, appendFileSync } = require('fs')
const { resolve } = require('path')
const { cwd } = require('process')
const confirmExit = require('./confirmExit')

module.exports = function (result) {
  if (Object.prototype.toString.call(result) !== '[object Object]') {
    confirmExit('生成报告异常，请重新运行程序…')
    return
  }

  // 清空上次记录
  const reportFile = resolve(`${cwd()}/report.txt`)
  writeFileSync(reportFile, '')

  // 已写入过的标题
  const titles = []

  // 写入新的报告
  for (const key in result) {
    if (Object.hasOwnProperty.call(result, key)) {
      // 按归类好的仓库数据遍历
      const repoResult = result[key]
      for (const t in repoResult) {
        if (Object.hasOwnProperty.call(repoResult, t)) {
          // 已写入过的类别
          const categories = []

          list = repoResult[t]
          list.forEach((item, index) => {
            const { repo, category, msg } = item

            if (!titles.includes(repo)) {
              appendFileSync(reportFile, `【${repo}】\n`)
              titles.push(repo)
            }

            if (!categories.includes(category)) {
              appendFileSync(reportFile, `--- ${category} ---\n`)
              categories.push(category)
            }

            appendFileSync(reportFile, `${index + 1}、${msg}\n`)

            if (index === list.length - 1) {
              appendFileSync(reportFile, `\n`)
            }
          })
        }
      }
    }
  }

  confirmExit('报告生成完毕，请打开 report.txt 查看')
}
