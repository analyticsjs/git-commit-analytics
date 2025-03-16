import { appendFileSync, writeFileSync } from 'node:fs'
import { resolve } from 'node:path'
import confirmExit from './confirmExit.js'
import getBasePath from './getBasePath.js'

/**
 * Save Report
 *
 * @param {object} result - The result transformed from config
 * @param {boolean} isEN - Check whether the language is English.
 */
export default function ({ result, isEN }) {
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
  const reportFile = resolve(getBasePath(), 'report.txt')
  writeFileSync(reportFile, '')

  // Titles that has been written
  const titles = []

  // Write new report
  for (const [, repoResult] of Object.entries(result)) {
    for (const [, list] of Object.entries(repoResult)) {
      // Categories that have been written
      const categories = []

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

  confirmExit({
    msg: isEN
      ? 'The report is generated successfully, please open "report.txt" to view'
      : '报告生成完毕，请打开 "report.txt" 查看',
    isEN,
  })
}
