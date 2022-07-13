const { writeFileSync, appendFileSync } = require('fs')
const { resolve } = require('path')
const { cwd } = require('process')
const confirmExit = require('./confirmExit')

/**
 * Save Report
 * @param {object} result - The result transformed from config
 * @param {boolean} isEN - Check whether the language is English.
 */
module.exports = function ({ result, isEN }) {
  if (Object.prototype.toString.call(result) !== '[object Object]') {
    confirmExit({
      msg: isEN
        ? 'An exception occurred when generating the report, please re-run the program…'
        : '生成报告时出现异常，请重新运行程序…',
      isEN,
    })
    return
  }

  // Clear the last record
  const reportFile = resolve(`${cwd()}/report.txt`)
  writeFileSync(reportFile, '')

  // Titles that has been written
  const titles = []

  // Write new report
  for (const key in result) {
    if (Object.hasOwnProperty.call(result, key)) {
      // 按归类好的仓库数据遍历
      const repoResult = result[key]
      for (const t in repoResult) {
        if (Object.hasOwnProperty.call(repoResult, t)) {
          // Categories that has been written
          const categories = []

          const list = repoResult[t]
          list.forEach((item, index) => {
            const { repo, category, msg } = item

            // Repo name as `<h2 />`
            if (!titles.includes(repo)) {
              appendFileSync(reportFile, `## ${repo}\n\n`)
              titles.push(repo)
            }

            // Category as `<h3 />`
            if (!categories.includes(category)) {
              appendFileSync(reportFile, `### ${category}\n`)
              categories.push(category)
            }

            // Commit message as `<li />`
            appendFileSync(reportFile, `${index + 1}. ${msg}\n`)

            if (index === list.length - 1) {
              appendFileSync(reportFile, `\n`)
            }
          })
        }
      }
    }
  }

  confirmExit({
    msg: isEN
      ? 'The report is generated successfully, please open "report.txt" to view'
      : '报告生成完毕，请打开 "report.txt" 查看',
    isEN,
  })
}
