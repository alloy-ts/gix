import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import { afterEach, beforeEach, expect, test } from "vite-plus/test";
import { Repository, Signature } from "./main.ts";

let tmpDir: string;

beforeEach(() => {
  tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), "gix-repo-test-"));
});

afterEach(() => {
  if (tmpDir && fs.existsSync(tmpDir)) {
    fs.rmSync(tmpDir, { recursive: true, force: true });
  }
});

test("Repository.init and Repository.open", () => {
  const repo = Repository.init(tmpDir);
  expect(repo.isBare()).toBe(false);
  expect(repo.isEmpty()).toBe(true);
  expect(repo.path()).toContain(".git");

  const opened = Repository.open(tmpDir);
  expect(opened.isBare()).toBe(false);
  expect(opened.headName()).toBe("refs/heads/main");
});

test("Repository.initBare creates bare repository", () => {
  const bareDir = path.join(tmpDir, "bare.git");
  const repo = Repository.initBare(bareDir);
  expect(repo.isBare()).toBe(true);
  expect(repo.workdir()).toBeNull();
});

test("Repository.discover finds repository in parent directory", () => {
  Repository.init(tmpDir);
  const subDir = path.join(tmpDir, "a", "b", "c");
  fs.mkdirSync(subDir, { recursive: true });

  const repo = Repository.discover(subDir);
  expect(repo.isBare()).toBe(false);
  expect(repo.path()).toContain(".git");
});

test("Signature creates author/committer signature", () => {
  const sig = new Signature("Alice", "alice@example.com");
  expect(sig.name()).toBe("Alice");
  expect(sig.email()).toBe("alice@example.com");
});
