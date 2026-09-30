import fs from "node:fs";
import path from "node:path";
import { createBuildCommand, NapiCli } from "@napi-rs/cli";

const build = createBuildCommand(process.argv.slice(2));
const options = build.getOptions();
const cli = new NapiCli();

const { task } = await cli.build({
  ...options,
  outputDir: "artifacts",
  cargoOptions: build.cargoOptions,
});

await task;

if (fs.existsSync("artifacts")) {
  for (const file of fs.readdirSync("artifacts")) {
    fs.copyFileSync(path.join("artifacts", file), path.join(".", file));
  }
}
