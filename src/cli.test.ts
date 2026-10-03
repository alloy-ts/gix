import { execSync } from "node:child_process";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import { afterEach, beforeEach, expect, test } from "vite-plus/test";
import { cliClone, cliInit, cliMain, cliStatus } from "./main.ts";

let tmpDir: string;

beforeEach(() => {
  tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), "gix-cli-test-"));
});

afterEach(() => {
  if (tmpDir && fs.existsSync(tmpDir)) {
    fs.rmSync(tmpDir, { recursive: true, force: true });
  }
});

test("cliInit initializes repository via NAPI handler", () => {
  const repoDir = path.join(tmpDir, "repo");
  const result = cliInit(repoDir, false);
  expect(result).toContain("Initialized Git repository");
  expect(fs.existsSync(path.join(repoDir, ".git"))).toBe(true);
});

test("cliStatus checks repository status", () => {
  cliInit(tmpDir, false);
  const status = cliStatus(tmpDir);
  expect(status).toContain("On branch refs/heads/main");
});

test("cliClone clones repository", () => {
  const targetDir = path.join(tmpDir, "cloned");
  const result = cliClone("https://example.com/repo.git", targetDir);
  expect(result).toContain("Cloned repository");
  expect(fs.existsSync(path.join(targetDir, ".git"))).toBe(true);
});

test("cliMain dispatches subcommands", () => {
  const repoDir = path.join(tmpDir, "mainRepo");
  const initRes = cliMain(["init", repoDir]);
  expect(initRes).toContain("Initialized Git repository");

  const statusRes = cliMain(["status", repoDir]);
  expect(statusRes).toContain("On branch refs/heads/main");
});

test("CLI execution via node child_process", () => {
  const repoDir = path.join(tmpDir, "childProcRepo");
  const evalScript = `
    const { cliInit, cliStatus } = require('./index.js');
    const dir = process.argv[1] || process.argv[2];
    cliInit(dir, false);
    console.log(cliStatus(dir));
  `;
  const output = execSync(`node -e "${evalScript.replace(/\n/g, " ")}" "${repoDir}"`, {
    encoding: "utf8",
  });
  expect(output).toContain("On branch refs/heads/main");
});
