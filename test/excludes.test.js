const getConfig = require('../src/libs/getConfig')
const config = getConfig()
const { excludes } = config
const reg = new RegExp(excludes.join('|'), 'im')

describe('Test Exclude Regexp', () => {
  test('chore: typo', () => {
    expect(reg.test('chore: typo')).toBeTruthy()
  })
})

describe('Test Exclude Regexp', () => {
  test('fix: bbbbbbbbb', () => {
    expect(reg.test('fix: bbbbbbbbb')).toBeFalsy()
  })
})

describe('Test Exclude Regexp', () => {
  test('chore: backup', () => {
    expect(reg.test('chore: backup')).toBeTruthy()
  })
})

describe('Test Exclude Regexp', () => {
  test('chore: save the coding progress', () => {
    expect(reg.test('chore: save the coding progress')).toBeTruthy()
  })
})
