import { execSync } from 'node:child_process'
import { platform } from 'node:os'
import { resolve } from 'node:path'
import confirmExit from './libs/confirmExit.js'
import formatLog from './libs/formatLog.js'
import getConfig from './libs/getConfig.js'
import getRepoName from './libs/getRepoName.js'
import saveReport from './libs/saveReport.js'

function start() {
  console.log('')
  console.log('node version:', execSync('node -v').toString())
  console.log('')

  try {
    const config = getConfig()
    if (!config) return
    const { lang, authors, dateRange, repos, format, includes, excludes } =
      config
    const isEN = lang === 'en'
    const startTime = dateRange[0]
    const endTime = dateRange[1]
    const result = {}

    console.log(
      isEN
        ? `Analyzing the Log of ${repos.length} repo${
            repos.length > 1 ? 's' : ''
          }, please be patient...`
        : `正在分析 ${repos.length} 个仓库的 Log ，请耐心等待…`,
    )

    // Create regular expression
    const reg = {
      author: new RegExp(authors.join('|'), 'im'),
      include: new RegExp(includes.join('|'), 'im'),
      exclude: new RegExp(excludes.join('|'), 'im'),
    }

    // The reports use repo to split paragraphs
    const allLogs = []
    repos.forEach((repo) => {
      // Get the repo name
      const repoName = getRepoName(repo, format)
      if (!result[repoName]) {
        result[repoName] = {
          feat: [],
          fix: [],
          docs: [],
          style: [],
          refactor: [],
          test: [],
          chore: [],
        }
      }

      // Create CMDs
      const commands = [
        `cd ${resolve(repo)}`,
        'git pull',
        `git log --pretty=format:"%an|||%ae|||%s|||'%h|||%ad"`,
      ]

      // Windows may have multiple disk partitions
      // Change the disk path when repo and program are on different disks
      if (platform() === 'win32') {
        const curDiskSymbol = process.argv0.split(':')[0]
        if (!String(repo).startsWith(curDiskSymbol)) {
          const diskSymbol = String(repo).split(':')[0]
          commands.unshift(`${diskSymbol}:`)
        }
      }

      // Gel full command
      const cmd = commands.join(' && ')

      // Get commit records from git repo
      const res = execSync(cmd)
      const str = String(res)
      const logs = str
        .split('\n')
        .filter((log) => reg.author.test(log))
        .filter((log) => reg.include.test(log))
        .filter((log) => !reg.exclude.test(log))

      // Merge all logs
      logs.forEach((log) => allLogs.push(`${repoName}|||${log}`))
    })

    // Deduplicate Logs
    const uniqueLogs = [...new Set(allLogs)]

    // Get target data
    const targetList = uniqueLogs
      .map((log) => formatLog({ log, isEN }))
      .filter((log) => {
        const { unix } = log
        return unix >= startTime && unix <= endTime
      })
      .sort((a, b) => {
        return a.unix - b.unix
      })

    // Classify
    targetList.forEach((item) => {
      const { repo, type, msg } = item

      // Filter duplicate messages
      const msgs = result[repo][type].map((i) => i.msg)
      if (msgs.includes(msg)) return

      // Add log record
      result[repo][type].push(item)
    })

    // Save
    saveReport({ result, isEN })
  } catch (e) {
    confirmExit(e)
  }
}
start()
