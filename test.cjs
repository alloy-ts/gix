const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const os = require("node:os");
const test = require("node:test");

const { Repository } = require("./index.js");

test("Repository.init creates a repository", () => {
  const tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), "git2-init-test-"));
  try {
    const repo = Repository.init(tmpDir);
    assert.equal(typeof repo, "object");
    assert.equal(repo.isBare(), false);
    assert.equal(repo.isEmpty(), true);
    assert.ok(repo.path().includes(".git"));
    assert.ok(repo.workdir());
  } finally {
    fs.rmSync(tmpDir, { recursive: true, force: true });
  }
});

test("Repository.open opens an existing repository", () => {
  const tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), "git2-open-test-"));
  try {
    Repository.init(tmpDir);
    const repo = Repository.open(tmpDir);
    assert.equal(typeof repo, "object");
    assert.equal(repo.isBare(), false);
  } finally {
    fs.rmSync(tmpDir, { recursive: true, force: true });
  }
});

test("Repository.clone clones a repository", () => {
  const srcDir = fs.mkdtempSync(path.join(os.tmpdir(), "git2-src-test-"));
  const destDir = fs.mkdtempSync(path.join(os.tmpdir(), "git2-clone-test-"));
  try {
    Repository.init(srcDir);
    const repo = Repository.clone(srcDir, destDir);
    assert.equal(typeof repo, "object");
    assert.equal(repo.isBare(), false);
  } finally {
    fs.rmSync(srcDir, { recursive: true, force: true });
    fs.rmSync(destDir, { recursive: true, force: true });
  }
});
