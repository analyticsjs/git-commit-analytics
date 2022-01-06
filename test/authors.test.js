const getConfig = require('../src/libs/getConfig')
const config = getConfig()
const { authors } = config
const reg = new RegExp(authors.join('|'), 'im')

describe('Test Author Regexp', () => {
  test('chengpeiquan', () => {
    expect(reg.test('chengpeiquan')).toBeTruthy()
  })
})

describe('Test Author Regexp', () => {
  test('ChengPeiQuan', () => {
    expect(reg.test('ChengPeiQuan')).toBeTruthy()
  })
})

describe('Test Author Regexp', () => {
  test('peiquan', () => {
    expect(reg.test('peiquan')).toBeFalsy()
  })
})

describe('Test Author Regexp', () => {
  test('petter', () => {
    expect(reg.test('petter')).toBeFalsy()
  })
})
