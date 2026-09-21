import { readFileSync } from 'node:fs'
import typescript from '@rollup/plugin-typescript'

const pkg = JSON.parse(readFileSync('package.json', 'utf8'))

export default {
	input: 'guest-js/index.ts',
	output: [
		{ file: pkg.exports['.'].import, format: 'esm' },
		{ file: pkg.exports['.'].require, format: 'cjs' },
	],
	plugins: [typescript({ declaration: true, declarationDir: 'dist-js' })],
	external: [/^@tauri-apps\/api/],
}
