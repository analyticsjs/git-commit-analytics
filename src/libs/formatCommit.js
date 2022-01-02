/**
 * Format Commit
 * @param {object} commit - A commit object
 * @param {boolean} isEN - Check whether the language is English.
 * @returns
 *  type: the commit type
 *  category: the text about commit type
 *  msg: the commit message
 */
module.exports = function ({ commit, isEN }) {
  // Get the text according to the type
  let type = 'chore'
  let category = isEN ? 'Chores' : '其他优化'
  if (commit.startsWith('feat')) {
    type = 'feat'
    category = isEN ? 'Features' : '功能开发'
  }
  if (commit.startsWith('fix')) {
    type = 'fix'
    category = isEN ? 'Bug Fixes' : 'BUG修复'
  }
  if (commit.startsWith('docs')) {
    type = 'docs'
    category = isEN ? 'Documentation' : '完善文档'
  }
  if (commit.startsWith('style')) {
    type = 'style'
    category = isEN ? 'Optimized Style' : '优化样式'
  }
  if (commit.startsWith('refactor')) {
    type = 'refactor'
    category = isEN ? 'Refactored' : '代码重构'
  }
  if (commit.startsWith('test')) {
    type = 'test'
    category = isEN ? 'Test Cases' : '测试用例'
  }

  // Extract messages from the commit log
  let msg = commit
  const index = commit.indexOf(':')
  if (index > -1) {
    msg = commit.slice(index + 1)
  }

  return {
    type,
    category,
    msg,
  }
}
