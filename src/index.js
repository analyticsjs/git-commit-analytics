const { resolve } = require('path')
const { execSync } = require('child_process')
const getConfig = require('./libs/getConfig')
const formatLog = require('./libs/formatLog')
const getRepoName = require('./libs/getRepoName')
const saveReport = require('./libs/saveReport')
const confirmExit = require('./libs/confirmExit')

function start() {
  try {
    const config = getConfig()
    if (!config) return
    const { authors, dateRange, repos, format, includes, excludes } = config
    const startTime = dateRange[0]
    const endTime = dateRange[1]
    const result = {}
    console.log(`正在分析 ${repos.length} 个仓库的 Log ，请耐心等待…`)

    // 创建正则
    const reg = {
      author: new RegExp(authors.join('|'), 'gim'),
      include: new RegExp(includes.join('|'), 'gim'),
      exclude: new RegExp(excludes.join('|'), 'gim'),
    }

    // 遍历仓库
    const allLogs = []
    repos.forEach((repo) => {
      // 获取仓库名称
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

      // 要执行的命令
      const cmds = [
        `cd ${resolve(repo)}`,
        'git pull',
        `git log --pretty=format:"%an|||%ae|||%s|||'%h|||%ad"`,
      ]
      const cmd = cmds.join(' && ')

      // 获取Git操作记录
      const res = execSync(cmd)
      const str = String(res)
      const logs = str
        .split('\n')
        .filter((log) => reg.author.test(log))
        .filter((log) => reg.include.test(log))
        .filter((log) => !reg.exclude.test(log))

      // 合并记录
      logs.forEach((log) => allLogs.push(`${repoName}|||${log}`))
    })

    // 去重
    const uniqueLogs = [...new Set(allLogs)]

    // 提取目标数据
    const targetList = uniqueLogs
      .map((log) => formatLog(log))
      .filter((log) => {
        const { unix } = log
        return unix >= startTime && unix <= endTime
      })
      .sort((a, b) => {
        return a.unix - b.unix
      })

    // 归类
    targetList.forEach((item) => {
      const { repo, type } = item
      result[repo][type].push(item)
    })

    // 写入报告
    saveReport(result)
  } catch (e) {
    confirmExit(e)
  }
}
start()
