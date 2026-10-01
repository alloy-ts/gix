const assert = require("node:assert/strict");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const test = require("node:test");

const { Repository, Signature, Reference } = require("./index.js");

test("Repository init, open, and basic operations", () => {
  const tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), "git2-test-"));
  try {
    const repo = Repository.init(tmpDir);
    assert.ok(repo);
    assert.equal(repo.isBare(), false);
    assert.equal(repo.isEmpty(), true);
    assert.ok(repo.path().includes(".git"));
    assert.ok(repo.workdir());

    const opened = Repository.open(tmpDir);
    assert.ok(opened);
    assert.equal(opened.isBare(), false);

    repo.addIgnoreRule("*.log");
    assert.equal(repo.isPathIgnored("test.log"), true);
    assert.equal(repo.isPathIgnored("test.txt"), false);
  } finally {
    fs.rmSync(tmpDir, { recursive: true, force: true });
  }
});

test("Signature creation and properties", () => {
  const sig = Signature.now("Test User", "user@example.com");
  assert.ok(sig);
  assert.equal(sig.name(), "Test User");
  assert.equal(sig.email(), "user@example.com");
  assert.ok(sig.timeSeconds() > 0);
});

test("Repository config and index access", () => {
  const tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), "git2-cfg-test-"));
  try {
    const repo = Repository.init(tmpDir);
    const cfg = repo.config();
    assert.ok(cfg);
    cfg.setString("user.name", "NAPI User");
    assert.equal(cfg.getString("user.name"), "NAPI User");

    const idx = repo.index();
    assert.ok(idx);
    assert.equal(idx.len(), 0);

    const refdb = repo.refdb();
    assert.ok(refdb);
    refdb.compress();
  } finally {
    fs.rmSync(tmpDir, { recursive: true, force: true });
  }
});

test("Reference static functions and repository references", () => {
  assert.equal(Reference.isValidName("refs/heads/main"), true);
  assert.equal(Reference.isValidName("invalid ref name"), false);
  assert.equal(Reference.normalizeName("refs/heads/main"), "refs/heads/main");

  const tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), "git2-ref-test-"));
  try {
    const repo = Repository.init(tmpDir);
    const refs = repo.references();
    assert.ok(Array.isArray(refs));
    const names = repo.referenceNames();
    assert.ok(Array.isArray(names));
  } finally {
    fs.rmSync(tmpDir, { recursive: true, force: true });
  }
});
