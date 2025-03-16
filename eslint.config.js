// @ts-check

import {
  createGetConfigNameFactory,
  defineFlatConfig,
  imports,
  javascript,
  markdown,
} from '@bassist/eslint-config'

const getConfigName = createGetConfigNameFactory('git-commit-analytics')

export default defineFlatConfig(
  [
    ...imports,
    ...markdown,
    ...javascript,
    {
      name: getConfigName('overrides'),
      rules: {
        'no-console': 'off',
        'require-await': 'off',
      },
    },
    {
      name: getConfigName('ignore'),
      files: ['**/dist/**', '**/temp/**'],
    },
  ],
  { tailwindcssEnabled: false },
)
