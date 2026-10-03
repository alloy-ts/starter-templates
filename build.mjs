import { createBuildCommand, NapiCli } from '@napi-rs/cli'
import path from 'node:path'

const args = process.argv.slice(2)
const build = createBuildCommand(args)
const options = build.getOptions()
const cli = new NapiCli()

const { task } = await cli.build({
  ...options,
  outputDir: options.outputDir || path.resolve(process.cwd(), 'dist'),
  cargoOptions: build.cargoOptions,
})

await task
