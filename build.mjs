import { createBuildCommand, NapiCli } from '@napi-rs/cli'
import { copyFileSync, readdirSync, existsSync } from 'node:fs'
import { join } from 'node:path'

const build = createBuildCommand(process.argv.slice(2))
const options = build.getOptions()
const cli = new NapiCli()

const { task } = await cli.build({
  ...options,
  outputDir: 'artifacts',
  cargoOptions: build.cargoOptions,
})

await task

if (existsSync('artifacts')) {
  for (const file of readdirSync('artifacts')) {
    copyFileSync(join('artifacts', file), join('.', file))
  }
}
