import { execSync } from "node:child_process";
import fs from "node:fs";
import path from "node:path";
import process from "node:process";
import { NapiCli } from "@napi-rs/cli";

const DEFAULT_TARGET_MATRIX = [
  { target: "x86_64-apple-darwin", flags: "-x" },
  { target: "aarch64-apple-darwin", flags: "-x" },
  { target: "x86_64-pc-windows-msvc", flags: "-x" },
  { target: "i686-pc-windows-msvc", flags: "-x" },
  { target: "aarch64-pc-windows-msvc", flags: "-x" },
  { target: "x86_64-unknown-linux-gnu", flags: "--use-napi-cross" },
  { target: "aarch64-unknown-linux-gnu", flags: "--use-napi-cross" },
  { target: "x86_64-unknown-linux-musl", flags: "-x" },
  { target: "aarch64-unknown-linux-musl", flags: "-x" },
  { target: "armv7-unknown-linux-gnueabihf", flags: "--use-napi-cross" },
  { target: "powerpc64le-unknown-linux-gnu", flags: "--use-napi-cross" },
  { target: "s390x-unknown-linux-gnu", flags: "--use-napi-cross" },
  { target: "wasm32-wasip1-threads", flags: "-x" },
];

function getPackageTargets() {
  try {
    const pkgPath = path.resolve(process.cwd(), "package.json");
    const pkg = JSON.parse(fs.readFileSync(pkgPath, "utf-8"));
    const targets = pkg?.napi?.targets;
    if (Array.isArray(targets) && targets.length > 0) {
      return targets.map((t) => {
        const match = DEFAULT_TARGET_MATRIX.find((m) => m.target === t);
        return match || { target: t, flags: "-x" };
      });
    }
  } catch {
    // fallback
  }
  return DEFAULT_TARGET_MATRIX;
}

function parseArgs() {
  const args = process.argv.slice(2);

  const isRelease = args.includes("--release") || args.includes("-r");
  const useNapiCrossArg = args.includes("--use-napi-cross");
  const crossCompileArg = args.includes("--cross-compile") || args.includes("-x");
  const useCrossArg = args.includes("--use-cross");
  const buildAll = args.includes("--target-all") || args.includes("--all");
  const dryRun = args.includes("--dry-run");

  const targetIdx = args.findIndex((a) => a === "--target" || a === "-t");
  const targetFilter = targetIdx !== -1 && args[targetIdx + 1] ? args[targetIdx + 1] : null;

  return {
    targetFilter,
    isRelease,
    useNapiCrossArg,
    crossCompileArg,
    useCrossArg,
    buildAll,
    dryRun,
  };
}

async function runBuild() {
  const {
    targetFilter,
    isRelease,
    useNapiCrossArg,
    crossCompileArg,
    useCrossArg,
    buildAll,
    dryRun,
  } = parseArgs();

  const packageTargets = getPackageTargets();

  let targetsToBuild = [];

  if (targetFilter) {
    const match = packageTargets.find((t) => t.target === targetFilter);
    targetsToBuild = match ? [match] : [{ target: targetFilter, flags: "-x" }];
  } else if (buildAll || useCrossArg) {
    targetsToBuild = packageTargets;
  } else {
    targetsToBuild = [{ target: undefined, flags: "" }]; // Default host target
  }

  console.log(`\n🚀 Starting NAPI Build Pipeline (${targetsToBuild.length} target(s))...`);

  const cli = new NapiCli();

  for (const item of targetsToBuild) {
    const target = item.target;
    const isNapiCrossTarget =
      target &&
      (target.includes("gnueabihf") ||
        target.includes("powerpc") ||
        target.includes("s390x") ||
        item.flags.includes("use-napi-cross"));

    const useNapiCross = useNapiCrossArg || isNapiCrossTarget;
    const crossCompile = crossCompileArg || (Boolean(target) && !useNapiCross);
    const useCross = useCrossArg;

    console.log(`\n⚙️  Building target: ${target || "host default"}`);
    if (dryRun) {
      console.log(`   [Dry Run] Skipped execution.`);
      continue;
    }

    try {
      await cli.build({
        platform: true,
        esm: true,
        release: isRelease,
        target,
        useNapiCross,
        crossCompile,
        useCross,
        outputDir: "artifacts",
      });

      const artifactsDir = path.resolve(process.cwd(), "artifacts");
      if (fs.existsSync(artifactsDir)) {
        const files = fs.readdirSync(artifactsDir);
        for (const file of files) {
          if (file.endsWith(".node") || file.endsWith(".js") || file.endsWith(".wasm")) {
            fs.copyFileSync(path.join(artifactsDir, file), path.join(process.cwd(), file));
          }
        }
      }

      console.log(`✅ Target ${target || "host default"} built successfully.`);
    } catch (err) {
      console.error(`❌ Target ${target || "host default"} failed:`, err.message);
      if (!buildAll) {
        process.exit(1);
      }
    }
  }

  try {
    execSync("npx vp fmt", { stdio: "ignore" });
  } catch {
    // ignore
  }

  console.log(`\n✨ Build process complete!`);
}

runBuild().catch((err) => {
  console.error("Fatal error during build:", err);
  process.exit(1);
});
