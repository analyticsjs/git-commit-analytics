const getConfig = require('../src/libs/getConfig')
const config = getConfig()
const { includes } = config
const reg = new RegExp(includes.join('|'), 'im')

describe('Test Include Regexp', () => {
  test('fix: aaaaaaaaa', () => {
    expect(reg.test('fix: aaaaaaaaa')).toBeTruthy()
  })
})

describe('Test Include Regexp', () => {
  test('fix: bbbbbbbbb', () => {
    expect(reg.test('fix: bbbbbbbbb')).toBeTruthy()
  })
})

describe('Test Include Regexp', () => {
  test('fix: ccccccccc', () => {
    expect(reg.test('fix: ccccccccc')).toBeTruthy()
  })
})

describe('Test Include Regexp', () => {
  test('fix: ddddddddd', () => {
    expect(reg.test('fix: ddddddddd')).toBeTruthy()
  })
})
