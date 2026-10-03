const assert = require("node:assert/strict");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const test = require("node:test");

void test("gix Repository init and open", async () => {
  const { Repository, Signature } = await import("./index.js");
  const tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), "gix-cjs-test-"));
  try {
    const repo = Repository.init(tmpDir);
    assert.equal(repo.isBare(), false);
    assert.equal(repo.isEmpty(), true);

    const openedRepo = Repository.open(tmpDir);
    assert.equal(openedRepo.isBare(), false);
    assert.equal(openedRepo.headName(), "refs/heads/main");

    const sig = new Signature("Bob", "bob@example.com");
    assert.equal(sig.name(), "Bob");
    assert.equal(sig.email(), "bob@example.com");
  } finally {
    fs.rmSync(tmpDir, { recursive: true, force: true });
  }
});
