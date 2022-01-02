module.exports = function (commit) {
  // 提交类型
  let type = 'chore'
  let category = '代码优化'
  if (commit.startsWith('feat')) {
    type = 'feat'
    category = '功能开发'
  }
  if (commit.startsWith('fix')) {
    type = 'fix'
    category = 'BUG修复'
  }
  if (commit.startsWith('docs')) {
    type = 'docs'
    category = '完善文档'
  }
  if (commit.startsWith('style')) {
    type = 'style'
    category = '优化样式'
  }
  if (commit.startsWith('refactor')) {
    type = 'refactor'
    category = '代码重构'
  }
  if (commit.startsWith('test')) {
    type = 'test'
    category = '测试用例'
  }

  // 截取提交内容
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
