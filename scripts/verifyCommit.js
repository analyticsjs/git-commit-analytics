// Invoked on the commit-msg git hook by yorkie.

const fs = require('fs')
const msgPath = process.env.GIT_PARAMS
const msg = fs.readFileSync(msgPath, 'utf-8').trim()

const commitRE =
  /^(revert: )?(feat|fix|docs|dx|style|refactor|perf|test|workflow|build|ci|chore|types|wip|release)(\(.+\))?: .{1,50}/

if (!commitRE.test(msg)) {
  console.log()
  console.error(
    `   ERROR  invalid commit message format.\n\n
        Proper commit message format is required for automated changelog generation. Examples:\n\n
          feat(compiler): add 'comments' option\n
          fix(v-model): handle events on blur (close #28)\n\n
        See .github/commit-convention.md for more details.\n`
  )
  process.exit(1)
}
