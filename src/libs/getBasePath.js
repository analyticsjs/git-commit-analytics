import { dirname } from 'node:path'
import { cwd } from 'node:process'
import { fileURLToPath } from 'node:url'

export default function getBasePath() {
  const isSea = process.env.NODE_ENV !== 'development'

  let __filename
  let __currentDir

  // For CommonJS
  if (typeof __dirname !== 'undefined') {
    // eslint-disable-next-line no-self-assign
    __filename = __filename
    __currentDir = __dirname
  }
  // For ES Module
  else {
    __filename = fileURLToPath(import.meta.url)
    __currentDir = dirname(__filename)
  }

  // Use cwd() in development mode and __currentDir in SEA mode
  const basePath = isSea ? __currentDir : cwd()
  return basePath
}
