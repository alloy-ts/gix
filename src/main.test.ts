import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import { afterEach, beforeEach, expect, test } from "vite-plus/test";
import { Repository, Signature } from "./main.ts";

let tmpDir: string;

beforeEach(() => {
  tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), "gix-test-"));
});

afterEach(() => {
  if (tmpDir && fs.existsSync(tmpDir)) {
    fs.rmSync(tmpDir, { recursive: true, force: true });
  }
});

test("Repository.init creates a new git repository", () => {
  const repo = Repository.init(tmpDir);
  expect(repo.isBare()).toBe(false);
  expect(repo.isEmpty()).toBe(true);
  expect(repo.path()).toContain(".git");
});

test("Repository.initBare creates a bare repository", () => {
  const bareDir = path.join(tmpDir, "bare.git");
  const repo = Repository.initBare(bareDir);
  expect(repo.isBare()).toBe(true);
});

test("Repository.open opens an existing repository", () => {
  Repository.init(tmpDir);
  const repo = Repository.open(tmpDir);
  expect(repo.isBare()).toBe(false);
  expect(repo.headName()).toBe("refs/heads/main");
});

test("Signature creates author/committer signature", () => {
  const sig = new Signature("Alice", "alice@example.com");
  expect(sig.name()).toBe("Alice");
  expect(sig.email()).toBe("alice@example.com");
});
