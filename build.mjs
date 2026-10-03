import { createBuildCommand, NapiCli } from '@napi-rs/cli'
import { copyFileSync, existsSync } from 'node:fs'

const build = createBuildCommand(process.argv.slice(2))
const options = build.getOptions()
const cli = new NapiCli()

const { task } = await cli.build({
  ...options,
  cargoOptions: build.cargoOptions,
  outputDir: "artifacts",
})

await task

if (existsSync('artifacts/index.js')) {
  copyFileSync('artifacts/index.js', 'index.js')
}
if (existsSync('artifacts/index.d.ts')) {
  copyFileSync('artifacts/index.d.ts', 'index.d.ts')
}
if (existsSync('artifacts/lib-crypto.linux-x64-gnu.node')) {
  copyFileSync('artifacts/lib-crypto.linux-x64-gnu.node', 'lib-crypto.linux-x64-gnu.node')
}
