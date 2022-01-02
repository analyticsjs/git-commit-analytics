const { readFileSync } = require('fs')
const { resolve } = require('path')
const { cwd } = require('process')
const dayjs = require('dayjs')
const confirmExit = require('./confirmExit')

module.exports = function () {
  try {
    const configFile = resolve(`${cwd()}/config.json`)
    const configStr = readFileSync(configFile)
    if (!configStr) {
      return null
    }

    // 格式化配置
    const config = JSON.parse(configStr)
    const { dateRange } = config

    // 校验数据
    const keys = Object.keys(config)
    for (let i = 0; i < keys.length; i++) {
      const key = keys[i]
      const value = config[key]

      // 格式化是个对象
      if (key === 'format') {
        if (Object.prototype.toString.call(value) !== '[object Object]') {
          confirmExit(`${key} 必须是一个 { [key: string]: string } 对象`)
          return null
        }

        for (const k in value) {
          if (
            Object.hasOwnProperty.call(value, k) &&
            typeof value[k] !== 'string'
          ) {
            confirmExit(`${key} 的 ${k} 的值必须是一个 string 字符串`)
            return null
          }
        }
      }
      // 其他都是数组
      else {
        if (!Array.isArray(value)) {
          confirmExit(`${key} 必须是一个 string[] 数组`)
          return null
        }
        if (['authors', 'repos'].includes(key) && !value.length) {
          confirmExit(`${key} 不能为空`)
          return null
        }
        if (value.length) {
          for (let e = 0; e < value.length; e++) {
            if (typeof value[e] !== 'string') {
              confirmExit(`${key} 的每个 item 都必须是 string 格式`)
              return null
            }
          }
        }
      }
    }

    // 处理起止日期
    if (dateRange.length && dateRange.length !== 2) {
      confirmExit(`dateRange 只能有 2 个值，[开始日期, 结束日期]`)
      return null
    }
    let startTime = dayjs().startOf('day').unix() * 1000
    let endTime = dayjs().endOf('day').unix() * 1000
    if (dayjs(dateRange[0]).isValid() && dayjs(dateRange[1]).isValid()) {
      startTime = dayjs(dateRange[0]).startOf('day').unix() * 1000
      endTime = dayjs(dateRange[1]).endOf('day').unix() * 1000
    }
    config.dateRange = [startTime, endTime]

    return config
  } catch (e) {
    confirmExit(e)
    return null
  }
}
