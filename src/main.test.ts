import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import { afterEach, beforeEach, expect, test } from "vite-plus/test";
import { Reference, Repository, Signature, messagePrettify } from "./main.ts";

let tmpDir: string;

beforeEach(() => {
  tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), "git2-test-"));
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
});

test("Repository ignore rules and status", () => {
  const repo = Repository.init(tmpDir);
  repo.addIgnoreRule("*.log");
  expect(repo.isPathIgnored("test.log")).toBe(true);
  expect(repo.isPathIgnored("test.txt")).toBe(false);
});

test("Signature creates author/committer signature", () => {
  const sig = Signature.now("Alice", "alice@example.com");
  expect(sig.name()).toBe("Alice");
  expect(sig.email()).toBe("alice@example.com");
});

test("messagePrettify cleans up commit messages", () => {
  const result = messagePrettify("  hello world  \n# comment\n", "#");
  expect(result.trim()).toBe("hello world");
});

test("Reference name validation", () => {
  expect(Reference.isValidName("refs/heads/main")).toBe(true);
  expect(Reference.isValidName("invalid..name")).toBe(false);
});

test("TreeBuilder and Odb write", () => {
  const repo = Repository.init(tmpDir);
  const odb = repo.odb();
  const blobOid = odb.write(3, Buffer.from("hello world"));
  expect(blobOid).toBeTruthy();
  expect(odb.exists(blobOid)).toBe(true);

  const tb = repo.treebuilder();
  tb.insert("hello.txt", blobOid, 0o100644);
  expect(tb.len()).toBe(1);
  const treeOid = tb.write();
  expect(treeOid).toBeTruthy();
});
