import { execSync } from "node:child_process";
import fs from "node:fs";
import path from "node:path";
import process from "node:process";
import { NapiCli } from "@napi-rs/cli";

const pkgPath = path.resolve(process.cwd(), "package.json");
const pkg = JSON.parse(fs.readFileSync(pkgPath, "utf-8"));
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

const pkgNapiTargets = (pkg.napi?.targets || []).map((t) => {
  if (typeof t === "string") {
    const match = DEFAULT_TARGET_MATRIX.find((m) => m.target === t);
    return match || { target: t, flags: "-x" };
  }
  return t;
});

function parseArgs() {
  const args = process.argv.slice(2);

  const buildAll =
    args.includes("--target-all") || args.includes("--all") || args.includes("--use-cross");
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

  return {
    targetFilter,
    isRelease,
    dryRun,
    buildAll,
    useCross,
    useNapiCross,
    crossCompile,
  };
}

async function runBuild() {
  const { targetFilter, isRelease, dryRun, buildAll, useCross, useNapiCross, crossCompile } =
    parseArgs();

  const cli = new NapiCli();

  let targetsToBuild = [];

  if (targetFilter) {
    const match = pkgNapiTargets.find((t) => t.target === targetFilter);
    targetsToBuild = match ? [match] : [{ target: targetFilter, flags: "-x" }];
  } else if (buildAll) {
    targetsToBuild = pkgNapiTargets.length > 0 ? pkgNapiTargets : DEFAULT_TARGET_MATRIX;
  } else {
    targetsToBuild = [{ target: null, flags: "" }];
  }

  console.log(`\n🚀 Starting Local Build Pipeline (${targetsToBuild.length} target(s))...`);

  for (const item of targetsToBuild) {
    const target = item.target;
    const isNapiCrossTarget =
      target &&
      (target.includes("gnueabihf") ||
        target.includes("powerpc") ||
        target.includes("s390x") ||
        (item.flags || "").includes("use-napi-cross"));

    const effectiveUseNapiCross = useNapiCross || (target ? isNapiCrossTarget : false);
    const effectiveCrossCompile = effectiveUseNapiCross
      ? false
      : crossCompile || (target ? !isNapiCrossTarget : false);

    console.log(`\n⚙️  Building target: ${target || "default host"}`);
    if (dryRun) {
      console.log(`   [Dry Run] Skipped build execution for ${target || "default host"}.`);
      continue;
    }

    try {
      const buildResult = await cli
        .build({
          platform: true,
          esm: true,
          release: isRelease,
          target: target || undefined,
          useNapiCross: effectiveUseNapiCross,
          crossCompile: effectiveCrossCompile,
          useCross,
          outputDir: "dist",
        })
        .catch((e) => {
          throw e;
        });

      if (buildResult && buildResult.task) {
        await buildResult.task;
      }

      const distDir = path.resolve(process.cwd(), "dist");
      if (fs.existsSync(distDir)) {
        const files = fs.readdirSync(distDir);
        for (const file of files) {
          if (file.endsWith(".node") || file.endsWith(".js") || file.endsWith(".wasm")) {
            fs.copyFileSync(path.join(distDir, file), path.join(process.cwd(), file));
          }
        }
      }

      console.log(`✅ Target ${target || "default host"} built successfully.`);
    } catch (err) {
      console.error(`❌ Target ${target || "default host"} failed to build:`, err?.message || err);
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
