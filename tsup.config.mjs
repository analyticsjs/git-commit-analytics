import { defineConfig } from 'tsup'

export default defineConfig({
  entry: ['src/index.js'],
  target: ['es2020'],
  format: 'cjs',
  outDir: 'temp',
  noExternal: ['dayjs'],
  bundle: true,
  minify: false,
  clean: true,
})
