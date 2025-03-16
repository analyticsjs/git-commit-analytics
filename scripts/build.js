import { execSync } from 'node:child_process'
import { existsSync, mkdirSync } from 'node:fs'
import { join } from 'node:path'

const WIN_OUTPUT_NAME = 'Git_Commit_Analytics_win.exe'
const MAC_OUTPUT_NAME = 'Git_Commit_Analytics_mac'
const SEA_BLOB = 'sea-prep.blob'
const OUTPUT_DIR = 'dist'

const NODE_PATH = process.execPath // Node.js executable file path

async function buildWin() {
  const OUTPUT_PATH = join(OUTPUT_DIR, WIN_OUTPUT_NAME)

  try {
    console.log(`📦 Creating standalone executable for Windows...`, OUTPUT_PATH)

    execSync(
      `node -e "require('fs').copyFileSync(process.execPath, '${OUTPUT_PATH}')" `,
    )

    try {
      execSync(`signtool remove /s ${OUTPUT_PATH}`)
    } catch {}

    execSync(
      [
        `npx postject ${OUTPUT_PATH} NODE_SEA_BLOB ${SEA_BLOB}`,
        '--sentinel-fuse NODE_SEA_FUSE_fce680ab2cc467b6e072b8b5df1996b2',
      ].join(' '),
    )

    execSync(`codesign --sign - ${OUTPUT_PATH}`)
    return true
  } catch (error) {
    console.error('❌ Failed to create executable for Windows:', error)
    return false
  }
}

async function buildMac() {
  const OUTPUT_PATH = join(OUTPUT_DIR, MAC_OUTPUT_NAME)

  try {
    console.log(`📦 Creating standalone executable for macOS...`, OUTPUT_PATH)

    execSync(`cp ${NODE_PATH} ${OUTPUT_PATH}`)

    execSync(`codesign --remove-signature ${OUTPUT_PATH}`)

    execSync(
      [
        `npx postject ${OUTPUT_PATH} NODE_SEA_BLOB ${SEA_BLOB}`,
        '--sentinel-fuse NODE_SEA_FUSE_fce680ab2cc467b6e072b8b5df1996b2',
        '--macho-segment-name NODE_SEA',
      ].join(' '),
    )

    execSync(`codesign --sign - ${OUTPUT_PATH}`)
    return true
  } catch (error) {
    console.error('❌ Failed to create executable for macOS:', error)
    return false
  }
}

// https://nodejs.org/api/single-executable-applications.html
async function build() {
  const PLATFORM = process.platform // 'win32' | 'darwin' | 'linux'
  const IS_WIN = PLATFORM === 'win32'

  if (!existsSync(OUTPUT_DIR)) {
    mkdirSync(OUTPUT_DIR, { recursive: true })
  }

  console.log('🔧 Generating SEA Blob...')
  try {
    execSync('node --experimental-sea-config sea-config.json', {
      stdio: 'inherit',
    })
  } catch (error) {
    console.error('❌ SEA Blob generation failed!', error)
    process.exit(1)
  }

  if (!existsSync(SEA_BLOB)) {
    console.error('❌ SEA Blob file not found!')
    process.exit(1)
  }

  const buildTask = IS_WIN ? buildWin : buildMac

  const isSuccess = await buildTask()
  if (!isSuccess) {
    process.exit(1)
  }

  console.log(`✅ Build complete!`)
}

build().catch(console.error)
