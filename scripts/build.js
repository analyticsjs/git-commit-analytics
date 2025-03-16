import { execSync } from 'node:child_process'
import { existsSync, mkdirSync } from 'node:fs'
import { join } from 'node:path'

// https://nodejs.org/api/single-executable-applications.html
async function build() {
  const WIN_OUTPUT_NAME = 'Git_Commit_Analytics_win.exe'
  const MAC_OUTPUT_NAME = 'Git_Commit_Analytics_mac'
  const SEA_BLOB = 'sea-prep.blob'
  const OUTPUT_DIR = 'dist'

  const PLATFORM = process.platform // 'win32' | 'darwin' | 'linux'
  const IS_WIN = PLATFORM === 'win32'
  const IS_MAC = PLATFORM === 'darwin'

  const OUTPUT_NAME = IS_WIN ? WIN_OUTPUT_NAME : MAC_OUTPUT_NAME
  const OUTPUT_PATH = join(OUTPUT_DIR, OUTPUT_NAME)
  const NODE_PATH = process.execPath // Node.js executable file path

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

  console.log(`📦 Creating standalone executable for ${PLATFORM}...`)

  try {
    if (IS_WIN) {
      execSync(
        `cmd /c "copy /b ${NODE_PATH} + ${SEA_BLOB} ${OUTPUT_PATH} && exit /b"`,
        {
          stdio: 'inherit',
          shell: true,
        },
      )
    } else {
      execSync(`cp ${NODE_PATH} ${OUTPUT_PATH}`)

      if (IS_MAC) {
        execSync(`codesign --remove-signature ${OUTPUT_PATH}`)

        execSync(
          [
            `npx postject ${OUTPUT_PATH} NODE_SEA_BLOB ${SEA_BLOB}`,
            '--sentinel-fuse NODE_SEA_FUSE_fce680ab2cc467b6e072b8b5df1996b2',
            '--macho-segment-name NODE_SEA',
          ].join(' '),
        )

        execSync(`codesign --sign - ${OUTPUT_PATH}`)
      }
    }
  } catch (error) {
    console.error('❌ Failed to create executable:', error)
    process.exit(1)
  }

  console.log(`✅ Build complete: ${OUTPUT_PATH}`)
}

build().catch(console.error)
