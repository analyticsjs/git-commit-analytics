import { resolve, sep } from 'node:path'

/**
 * Get Repo Name
 *
 * @param {any} repoDir - The item in configuration `config.repos`
 * @param {any} format - The configuration `config.format`
 * @returns The repository name
 */
export default function (repoDir, format) {
  const arr = resolve(repoDir).split(sep)
  if (!arr.length) {
    return 'undefined'
  }

  const i = arr.length - 1
  const key = arr[i]
  return format[key]
}
