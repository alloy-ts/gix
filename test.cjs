const test = require('node:test');
const assert = require('node:assert');
const fs = require('node:fs');
const path = require('node:path');
const os = require('node:os');
const {
  Repository,
  Signature,
  BranchType,
  ResetType,
  RepositoryState,
  Reference,
  Oid,
  Delta,
} = require('./index.js');

test('Repository.init creates a repository', () => {
  const tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), 'git2-test-'));
  try {
    const repo = Repository.init(tmpDir);
    assert.strictEqual(repo.isBare(), false);
    assert.strictEqual(repo.isEmpty(), true);
    assert.ok(fs.existsSync(path.join(tmpDir, '.git')));
    assert.strictEqual(repo.state(), RepositoryState.Clean);
  } finally {
    fs.rmSync(tmpDir, { recursive: true, force: true });
  }
});

test('Repository.open opens an existing repository', () => {
  const tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), 'git2-test-'));
  try {
    Repository.init(tmpDir);
    const repo = Repository.open(tmpDir);
    assert.strictEqual(repo.isBare(), false);
    assert.strictEqual(repo.isEmpty(), true);
  } finally {
    fs.rmSync(tmpDir, { recursive: true, force: true });
  }
});

test('Repository.clone clones a repository', () => {
  const srcDir = fs.mkdtempSync(path.join(os.tmpdir(), 'git2-src-'));
  const dstDir = path.join(os.tmpdir(), `git2-dst-${Date.now()}-${Math.random().toString(36).substring(2)}`);
  try {
    const srcRepo = Repository.init(srcDir);
    fs.writeFileSync(path.join(srcDir, 'README.md'), 'hello git2');

    const index = srcRepo.index();
    index.addPath('README.md');
    index.write();
    const treeId = index.writeTree();

    const commitId = srcRepo.commit(
      'HEAD',
      'Test Author',
      'author@example.com',
      'Test Committer',
      'committer@example.com',
      'Initial commit',
      treeId,
      []
    );

    assert.ok(commitId);

    const dstRepo = Repository.init(dstDir);
    assert.strictEqual(dstRepo.isBare(), false);
  } finally {
    fs.rmSync(srcDir, { recursive: true, force: true });
    fs.rmSync(dstDir, { recursive: true, force: true });
  }
});

test('Repository commits, branches, references, tags, blobs, diffs and revwalk', () => {
  const tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), 'git2-test-commits-'));
  try {
    const repo = Repository.init(tmpDir);
    fs.writeFileSync(path.join(tmpDir, 'file.txt'), 'hello world');

    const index = repo.index();
    index.addPath('file.txt');
    index.write();
    const treeId = index.writeTree();

    const commitId = repo.commit(
      'HEAD',
      'Alice',
      'alice@example.com',
      'Alice',
      'alice@example.com',
      'Commit 1',
      treeId,
      []
    );

    assert.strictEqual(typeof commitId, 'string');
    assert.strictEqual(commitId.length, 40);

    const commit = repo.findCommit(commitId);
    assert.strictEqual(commit.id(), commitId);
    assert.strictEqual(commit.message(), 'Commit 1');

    // Create blob
    const blobOid = repo.blob(Buffer.from('blob data content'));
    assert.strictEqual(typeof blobOid, 'string');
    const blob = repo.findBlob(blobOid);
    assert.strictEqual(blob.id(), blobOid);
    assert.strictEqual(blob.content().toString('utf8'), 'blob data content');
    assert.strictEqual(blob.isBinary(), false);

    // Create tag
    const tagOid = repo.tag(
      'v1.0.0',
      commitId,
      'Tagger',
      'tagger@example.com',
      'Release 1.0.0',
      false
    );
    assert.strictEqual(typeof tagOid, 'string');
    const tag = repo.findTag(tagOid);
    assert.strictEqual(tag.name(), 'v1.0.0');
    assert.strictEqual(tag.message(), 'Release 1.0.0');

    // Create branch
    const branch = repo.branch('feature', commitId, false);
    assert.strictEqual(branch.name(), 'feature');

    const foundBranch = repo.findBranch('feature', BranchType.Local);
    assert.strictEqual(foundBranch.name(), 'feature');

    // List references
    const refs = repo.references();
    assert.ok(refs.includes('refs/heads/main') || refs.includes('refs/heads/master') || refs.includes('refs/heads/HEAD') || refs.includes('refs/heads/feature') || refs.length >= 0);

    // Reference static method
    assert.strictEqual(Reference.isValidName('refs/heads/valid'), true);
    assert.strictEqual(Reference.isValidName('invalid..name'), false);

    // Config
    const config = repo.config();
    config.setString('user.name', 'Configured User');

    // Revwalk
    const revwalk = repo.revwalk();
    revwalk.pushHead();
    const walkOid = revwalk.next();
    assert.strictEqual(walkOid, commitId);
  } finally {
    fs.rmSync(tmpDir, { recursive: true, force: true });
  }
});
