const assert = require("node:assert/strict");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const test = require("node:test");

void test("git2 Repository init and open", async () => {
  const { Repository, Signature, Reference, ObjectType } = await import("./index.js");
  const tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), "git2-cjs-test-"));
  try {
    const repo = Repository.init(tmpDir);
    assert.equal(repo.isBare(), false);
    assert.equal(repo.isEmpty(), true);

    const openedRepo = Repository.open(tmpDir);
    assert.equal(openedRepo.isBare(), false);

    const sig = Signature.now("Bob", "bob@example.com");
    assert.equal(sig.name(), "Bob");
    assert.equal(sig.email(), "bob@example.com");

    assert.equal(Reference.isValidName("refs/heads/main"), true);

    const odb = repo.odb();
    const oid = odb.write(ObjectType.Blob, Buffer.from("test blob"));
    assert.equal(odb.exists(oid), true);
  } finally {
    fs.rmSync(tmpDir, { recursive: true, force: true });
  }
});
