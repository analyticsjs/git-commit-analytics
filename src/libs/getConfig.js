import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { cwd } from 'node:process'
import dayjs from 'dayjs'
import confirmExit from './confirmExit.js'

/**
 * Get Config
 *
 * Format content from configuration file and check validity.
 *
 * @returns An object from the configuration file
 * @tips The `dateRange` will be transformed from `string[]` to `number[]`
 */
export default function () {
  let isEN = true

  try {
    // Read content from configuration file
    const configFile = resolve(`${cwd()}/config.json`)
    const configStr = readFileSync(configFile)
    if (!configStr) {
      return null
    }

    const config = JSON.parse(configStr)
    const { dateRange } = config

    // Check validity
    const keys = Object.keys(config)
    for (let i = 0; i < keys.length; i++) {
      const key = keys[i]
      const value = config[key]

      // Check language, Set the default language to English
      if (key === 'lang') {
        if (!['en', 'zh'].includes(value)) {
          config.lang = 'en'
        }
      }
      isEN = config.lang === 'en'

      // Check object
      if (key === 'format') {
        if (Object.prototype.toString.call(value) !== '[object Object]') {
          confirmExit({
            msg: isEN
              ? `${key} must be an object as { [key: string]: string }`
              : `${key} 必须是一个 { [key: string]: string } 对象`,
            isEN,
          })
          return null
        }

        for (const [k, v] of Object.entries(value)) {
          if (typeof v !== 'string') {
            confirmExit({
              msg: isEN
                ? `The value of ${k} of ${key} must be a string`
                : `${key} 的 ${k} 的值必须是一个 string 字符串`,
              isEN,
            })
            return null
          }
        }
      }

      // Check array
      if (
        ['authors', 'dateRange', 'repos', 'includes', 'excludes'].includes(key)
      ) {
        if (!Array.isArray(value)) {
          confirmExit({
            msg: isEN
              ? `${key} must be a string[] array`
              : `${key} 必须是一个 string[] 数组`,
            isEN,
          })
          return null
        }
        if (['authors', 'repos'].includes(key) && !value.length) {
          confirmExit({
            msg: isEN ? `${key} cannot be empty` : `${key} 不能为空`,
            isEN,
          })
          return null
        }
        if (value.length) {
          for (let e = 0; e < value.length; e++) {
            if (typeof value[e] !== 'string') {
              confirmExit({
                msg: isEN
                  ? `Each item of ${key} must be a string`
                  : `${key} 的每个 item 都必须是 string 格式`,
                isEN,
              })
              return null
            }
          }
        }
      }
    }

    // Handle start date and end date, transform to timestamp
    if (dateRange.length && dateRange.length !== 2) {
      confirmExit({
        msg: isEN
          ? `dateRange can only have 2 values, [start date, end date]`
          : `dateRange 只能有 2 个值，[开始日期, 结束日期]`,
        isEN,
      })
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
    confirmExit({
      msg: e,
      isEN,
    })
    return null
  }
}
