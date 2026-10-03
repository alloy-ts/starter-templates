import fs from "node:fs";
import path from "node:path";
import process from "node:process";
import { createBuildCommand, NapiCli } from "@napi-rs/cli";

const pkgPath = path.resolve(process.cwd(), "package.json");
const pkg = JSON.parse(fs.readFileSync(pkgPath, "utf8"));
const pkgNapiTargets = pkg.napi?.targets || [];

function parseArgs() {
  const args = process.argv.slice(2);

  const buildAll = args.includes("--target-all") || args.includes("--all");
  const isRelease = args.includes("--release") || args.includes("-r");
  const dryRun = args.includes("--dry-run");

  const useCross = args.includes("--use-cross");
  const useNapiCross = args.includes("--use-napi-cross");
  const crossCompile = args.includes("-x") || args.includes("--cross-compile");

  let targetFilter = null;
  const targetIdx = args.indexOf("--target");
  if (targetIdx !== -1 && args[targetIdx + 1]) {
    targetFilter = args[targetIdx + 1];
  }

  const filteredArgs = args.filter(
    (arg) => arg !== "--target-all" && arg !== "--all" && arg !== "--dry-run",
  );

  return {
    targetFilter,
    isRelease,
    dryRun,
    buildAll,
    useCross,
    useNapiCross,
    crossCompile,
    filteredArgs,
  };
}

function copyArtifactsToRoot() {
  const artifactsDir = path.resolve(process.cwd(), "artifacts");
  if (!fs.existsSync(artifactsDir)) return;

  const files = fs.readdirSync(artifactsDir);
  for (const file of files) {
    if (file.endsWith(".node") || file.endsWith(".js") || file.endsWith(".d.ts")) {
      const src = path.join(artifactsDir, file);
      const dest = path.join(process.cwd(), file);
      fs.copyFileSync(src, dest);
    }
  }
}

async function runBuild() {
  const {
    targetFilter,
    isRelease,
    dryRun,
    buildAll,
    useCross,
    useNapiCross,
    crossCompile,
    filteredArgs,
  } = parseArgs();
  const buildCommand = createBuildCommand(filteredArgs);
  const options = buildCommand.getOptions();
  const cli = new NapiCli();

  let targetsToBuild = [];

  if (targetFilter) {
    targetsToBuild = [targetFilter];
  } else if (buildAll) {
    targetsToBuild = pkgNapiTargets.length > 0 ? pkgNapiTargets : [null];
  } else {
    targetsToBuild = [null];
  }

  console.log(`\n🚀 Starting Build Pipeline (${targetsToBuild.length} target(s))...`);

  for (const target of targetsToBuild) {
    const isNapiCrossTarget =
      target &&
      (target.includes("gnueabihf") || target.includes("powerpc") || target.includes("s390x"));
    const effectiveUseNapiCross = useNapiCross || (target ? isNapiCrossTarget : false);
    const effectiveCrossCompile = crossCompile || (target ? !isNapiCrossTarget : false);

    console.log(`\n⚙️  Building target: ${target || "default host"}`);
    if (dryRun) {
      console.log(`   [Dry Run] Skipped build execution for ${target || "default host"}.`);
      continue;
    }

    try {
      const buildOpts = {
        ...options,
        cwd: options.cwd || process.cwd(),
        outputDir: "artifacts",
        release: isRelease,
        cargoOptions: buildCommand.cargoOptions,
      };

      if (target) {
        buildOpts.target = target;
      }
      if (useCross) buildOpts.useCross = true;
      if (effectiveUseNapiCross) buildOpts.useNapiCross = true;
      if (effectiveCrossCompile) buildOpts.crossCompile = true;

      const { task } = await cli.build(buildOpts);
      await task;
      console.log(`✅ Target ${target || "default host"} built successfully.`);
    } catch (err) {
      console.error(`❌ Target ${target || "default host"} failed to build:`, err.message);
      if (!buildAll) {
        process.exit(1);
      }
    }
  }

  copyArtifactsToRoot();
  console.log(`\n✨ Build process complete!`);
}

runBuild().catch((err) => {
  console.error("Fatal error during build:", err);
  process.exit(1);
});
