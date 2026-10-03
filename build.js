import fs from "node:fs";
import path from "node:path";
import { NapiCli } from "@napi-rs/cli";

async function run() {
  const args = process.argv.slice(2);
  const isRelease = args.includes("--release") || args.includes("-r");
  const useNapiCross = args.includes("--use-napi-cross");
  const crossCompile = args.includes("--cross-compile") || args.includes("-x");
  const useCross = args.includes("--use-cross");

  const targetIdx = args.findIndex((a) => a === "--target" || a === "-t");
  const target = targetIdx !== -1 && args[targetIdx + 1] ? args[targetIdx + 1] : undefined;

  const cli = new NapiCli();
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

  // Ensure generated index files are formatted
  try {
    const { execSync } = await import("node:child_process");
    execSync("npx vp fmt", {
      stdio: "ignore",
    });
  } catch {
    // ignore
  }
}

void run().catch((err) => {
  console.error(err);
  process.exit(1);
});
