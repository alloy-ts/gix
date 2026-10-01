import fs from "node:fs";
import path from "node:path";

// Ensure @napi-rs/cli staging directory patch is applied in sandbox environments
try {
  const cliPath = path.resolve("node_modules/@napi-rs/cli/dist/index.js");
  if (fs.existsSync(cliPath)) {
    let content = fs.readFileSync(cliPath, "utf8");
    const target =
      "const stagingDir = await mkdtemp(join(dirname(finalOutputDir), `.${basename(finalOutputDir)}.napi-stage-`));";
    const replacement =
      "const stagingDir = await mkdtemp(join(finalOutputDir, `.${basename(finalOutputDir)}.napi-stage-`));";
    if (content.includes(target)) {
      fs.writeFileSync(cliPath, content.replace(target, replacement));
    }
  }
} catch {
  // Ignore errors
}

const { NapiCli } = await import("@napi-rs/cli");

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
  });
}

void run().catch((err) => {
  console.error(err);
  process.exit(1);
});
