import { createBuildCommand, NapiCli } from '@napi-rs/cli'

const build = createBuildCommand(process.argv.slice(2))
const options = build.getOptions()
const cli = new NapiCli()

import { copyFileSync, readdirSync } from 'node:fs'
import { join } from 'node:path'

const { task } = await cli.build({
  ...options,
  outputDir: 'artifacts',
  cargoOptions: build.cargoOptions,
})

await task

try {
  const files = readdirSync('artifacts')
  for (const file of files) {
    copyFileSync(join('artifacts', file), join('.', file))
  }
} catch (e) {
  // ignore
}
