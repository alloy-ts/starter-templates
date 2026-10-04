import fs from "node:fs";
import path from "node:path";
import { createBuildCommand, NapiCli } from "@napi-rs/cli";

const pkgPath = path.resolve(process.cwd(), "package.json");
const pkg = JSON.parse(fs.readFileSync(pkgPath, "utf8"));
const napiTargets = pkg.napi?.targets || [];

const rawArgs = process.argv.slice(2);
const isTargetAll = rawArgs.includes("--target-all");
const useCross = rawArgs.includes("--use-cross") || rawArgs.includes("--use-napi-cross");

const filteredArgs = rawArgs.filter(
  (arg) => arg !== "--target-all" && arg !== "--use-cross" && arg !== "--use-napi-cross",
);

async function buildTarget(args, target) {
  const targetArgs = [...args];
  if (target && !targetArgs.includes("--target")) {
    targetArgs.push("--target", target);
  }
  if (useCross && !targetArgs.includes("--use-napi-cross")) {
    targetArgs.push("--use-napi-cross");
  }

  const build = createBuildCommand(targetArgs);
  const options = build.getOptions();
  const cli = new NapiCli();

  const outputDir = "artifacts";
  const { task } = await cli.build({
    ...options,
    outputDir,
    cargoOptions: build.cargoOptions,
  });

  await task;

  // Copy generated .node files from artifacts directory to root
  const artifactsDir = path.resolve(process.cwd(), outputDir);
  if (fs.existsSync(artifactsDir)) {
    const files = fs.readdirSync(artifactsDir);
    for (const file of files) {
      if (file.endsWith(".node")) {
        fs.copyFileSync(path.join(artifactsDir, file), path.join(process.cwd(), file));
      }
    }
  }
}

if (isTargetAll) {
  for (const target of napiTargets) {
    console.log(`Building target: ${target}`);
    await buildTarget(filteredArgs, target);
  }
} else {
  await buildTarget(filteredArgs);
}
