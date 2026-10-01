const assert = require("node:assert/strict");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const test = require("node:test");

const { Repository } = require("./index.js");

test("Repository init, open, and methods", () => {
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
