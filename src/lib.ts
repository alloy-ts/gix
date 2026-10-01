import {
  Repository as NativeRepository,
  Signature as NativeSignature,
  Oid as NativeOid,
  Commit as NativeCommit,
  Tree as NativeTree,
  TreeEntry as NativeTreeEntry,
  Blob as NativeBlob,
  Tag as NativeTag,
  Branch as NativeBranch,
  Reference as NativeReference,
  Index as NativeIndex,
  StatusEntry as NativeStatusEntry,
  DiffFile as NativeDiffFile,
  DiffDelta as NativeDiffDelta,
  Diff as NativeDiff,
  Revwalk as NativeRevwalk,
  Worktree as NativeWorktree,
  Config as NativeConfig,
  RepositoryState,
  BranchType,
  ObjectType,
  ResetType,
  Delta,
} from './native.js';

export {
  NativeRepository,
  NativeSignature,
  NativeOid,
  NativeCommit,
  NativeTree,
  NativeTreeEntry,
  NativeBlob,
  NativeTag,
  NativeBranch,
  NativeReference,
  NativeIndex,
  NativeStatusEntry,
  NativeDiffFile,
  NativeDiffDelta,
  NativeDiff,
  NativeRevwalk,
  NativeWorktree,
  NativeConfig,
  RepositoryState,
  BranchType,
  ObjectType,
  ResetType,
  Delta,
};

export class Repository {
  private inner: NativeRepository;

  private constructor(inner: NativeRepository) {
    this.inner = inner;
  }

  static init(path: string): Repository {
    return new Repository(NativeRepository.init(path));
  }

  static initBare(path: string): Repository {
    return new Repository(NativeRepository.initBare(path));
  }

  static open(path: string): Repository {
    return new Repository(NativeRepository.open(path));
  }

  static openBare(path: string): Repository {
    return new Repository(NativeRepository.openBare(path));
  }

  static discover(path: string): Repository {
    return new Repository(NativeRepository.discover(path));
  }

  static clone(url: string, path: string): Repository {
    return new Repository(NativeRepository.clone(url, path));
  }

  isBare(): boolean {
    return this.inner.isBare();
  }

  isEmpty(): boolean {
    return this.inner.isEmpty();
  }

  path(): string {
    return this.inner.path();
  }

  workdir(): string | null {
    return this.inner.workdir();
  }

  state(): RepositoryState {
    return this.inner.state();
  }

  head(): Reference {
    return new Reference(this.inner.head());
  }

  setHead(refname: string): void {
    this.inner.setHead(refname);
  }

  headDetached(): boolean {
    return this.inner.headDetached();
  }

  references(): string[] {
    return this.inner.references();
  }

  referencesGlob(glob: string): string[] {
    return this.inner.referencesGlob(glob);
  }

  reference(name: string, targetOid: string, force: boolean, logMessage: string): Reference {
    return new Reference(this.inner.reference(name, targetOid, force, logMessage));
  }

  referenceSymbolic(name: string, target: string, force: boolean, logMessage: string): Reference {
    return new Reference(this.inner.referenceSymbolic(name, target, force, logMessage));
  }

  refnameToId(name: string): string {
    return this.inner.refnameToId(name);
  }

  config(): Config {
    return new Config(this.inner.config());
  }

  index(): Index {
    return new Index(this.inner.index());
  }

  revwalk(): Revwalk {
    return new Revwalk(this.inner.revwalk());
  }

  findCommit(oid: string): Commit {
    return new Commit(this.inner.findCommit(oid));
  }

  findTree(oid: string): Tree {
    return new Tree(this.inner.findTree(oid));
  }

  findBlob(oid: string): Blob {
    return new Blob(this.inner.findBlob(oid));
  }

  findTag(oid: string): Tag {
    return new Tag(this.inner.findTag(oid));
  }

  findBranch(name: string, branchType: BranchType): Branch {
    return new Branch(this.inner.findBranch(name, branchType));
  }

  findReference(name: string): Reference {
    return new Reference(this.inner.findReference(name));
  }

  blob(data: Buffer): string {
    return this.inner.blob(data);
  }

  branch(name: string, targetCommitId: string, force: boolean): Branch {
    return new Branch(this.inner.branch(name, targetCommitId, force));
  }

  commit(
    updateRef: string | null,
    authorName: string,
    authorEmail: string,
    committerName: string,
    committerEmail: string,
    message: string,
    treeId: string,
    parentIds: string[]
  ): string {
    return this.inner.commit(
      updateRef,
      authorName,
      authorEmail,
      committerName,
      committerEmail,
      message,
      treeId,
      parentIds
    );
  }

  tag(
    name: string,
    targetOid: string,
    taggerName: string,
    taggerEmail: string,
    message: string,
    force: boolean
  ): string {
    return this.inner.tag(name, targetOid, taggerName, taggerEmail, message, force);
  }

  checkoutHead(): void {
    this.inner.checkoutHead();
  }

  statuses(): StatusEntry[] {
    return this.inner.statuses().map((s) => new StatusEntry(s));
  }

  statusFile(path: string): number {
    return this.inner.statusFile(path);
  }

  diffTreeToTree(oldTreeId: string | null, newTreeId: string | null): Diff {
    return new Diff(this.inner.diffTreeToTree(oldTreeId, newTreeId));
  }

  worktrees(): string[] {
    return this.inner.worktrees();
  }

  findWorktree(name: string): Worktree {
    return new Worktree(this.inner.findWorktree(name));
  }
}

export class Signature {
  static now(name: string, email: string): NativeSignature {
    return NativeSignature.now(name, email);
  }
}

export class Oid {
  static fromStr(s: string): NativeOid {
    return NativeOid.fromStr(s);
  }
}

export class Commit {
  constructor(private inner: NativeCommit) {}

  id(): string {
    return this.inner.id();
  }

  message(): string | null {
    return this.inner.message();
  }

  summary(): string | null {
    return this.inner.summary();
  }

  time(): number {
    return this.inner.time();
  }

  author(): NativeSignature {
    return this.inner.author();
  }

  committer(): NativeSignature {
    return this.inner.committer();
  }

  treeId(): string {
    return this.inner.treeId();
  }

  parentCount(): number {
    return this.inner.parentCount();
  }

  parentId(i: number): string {
    return this.inner.parentId(i);
  }
}

export class Tree {
  constructor(private inner: NativeTree) {}

  id(): string {
    return this.inner.id();
  }

  len(): number {
    return this.inner.len();
  }

  isEmpty(): boolean {
    return this.inner.isEmpty();
  }

  getName(filename: string): TreeEntry | null {
    const entry = this.inner.getName(filename);
    return entry ? new TreeEntry(entry) : null;
  }
}

export class TreeEntry {
  constructor(private inner: NativeTreeEntry) {}

  id(): string {
    return this.inner.id;
  }

  name(): string | null {
    return this.inner.name;
  }

  filemode(): number {
    return this.inner.filemode;
  }
}

export class Blob {
  constructor(private inner: NativeBlob) {}

  id(): string {
    return this.inner.id();
  }

  content(): Buffer {
    return this.inner.content();
  }

  isBinary(): boolean {
    return this.inner.isBinary();
  }

  size(): number {
    return this.inner.size();
  }
}

export class Tag {
  constructor(private inner: NativeTag) {}

  id(): string {
    return this.inner.id();
  }

  name(): string | null {
    return this.inner.name();
  }

  message(): string | null {
    return this.inner.message();
  }

  targetId(): string {
    return this.inner.targetId();
  }
}

export class Branch {
  constructor(private inner: NativeBranch) {}

  name(): string | null {
    return this.inner.name();
  }

  isHead(): boolean {
    return this.inner.isHead();
  }

  getReference(): Reference {
    return new Reference(this.inner.getReference());
  }
}

export class Reference {
  constructor(private inner: NativeReference) {}

  static isValidName(refname: string): boolean {
    return NativeReference.isValidName(refname);
  }

  name(): string | null {
    return this.inner.name();
  }

  shorthand(): string | null {
    return this.inner.shorthand();
  }

  target(): string | null {
    return this.inner.target();
  }

  targetPeel(): string | null {
    return this.inner.targetPeel();
  }

  symbolicTarget(): string | null {
    return this.inner.symbolicTarget();
  }

  resolve(): Reference {
    return new Reference(this.inner.resolve());
  }

  rename(newName: string, force: boolean, logMessage: string): Reference {
    return new Reference(this.inner.rename(newName, force, logMessage));
  }

  delete(): void {
    this.inner.delete();
  }

  setTarget(targetOidStr: string, logMessage: string): Reference {
    return new Reference(this.inner.setTarget(targetOidStr, logMessage));
  }

  symbolicSetTarget(target: string, logMessage: string): Reference {
    return new Reference(this.inner.symbolicSetTarget(target, logMessage));
  }

  isBranch(): boolean {
    return this.inner.isBranch();
  }

  isRemote(): boolean {
    return this.inner.isRemote();
  }

  isTag(): boolean {
    return this.inner.isTag();
  }

  isNote(): boolean {
    return this.inner.isNote();
  }
}

export class Index {
  constructor(private inner: NativeIndex) {}

  addPath(path: string): void {
    this.inner.addPath(path);
  }

  removePath(path: string): void {
    this.inner.removePath(path);
  }

  write(): void {
    this.inner.write();
  }

  writeTree(): string {
    return this.inner.writeTree();
  }

  len(): number {
    return this.inner.len();
  }

  isEmpty(): boolean {
    return this.inner.isEmpty();
  }

  read(force: boolean): void {
    this.inner.read(force);
  }
}

export class StatusEntry {
  constructor(private inner: NativeStatusEntry) {}

  path(): string | null {
    return this.inner.path;
  }

  status(): number {
    return this.inner.status;
  }
}

export class DiffFile {
  constructor(private inner: NativeDiffFile) {}

  path(): string | null {
    return this.inner.path;
  }

  id(): string {
    return this.inner.id;
  }

  size(): number {
    return this.inner.size;
  }
}

export class DiffDelta {
  constructor(private inner: NativeDiffDelta) {}

  status(): Delta {
    return this.inner.status;
  }

  oldFile(): DiffFile {
    return new DiffFile(this.inner.oldFile);
  }

  newFile(): DiffFile {
    return new DiffFile(this.inner.newFile);
  }
}

export class Diff {
  constructor(private inner: NativeDiff) {}

  deltasLen(): number {
    return this.inner.deltasLen();
  }

  getDelta(idx: number): DiffDelta | null {
    const delta = this.inner.getDelta(idx);
    return delta ? new DiffDelta(delta) : null;
  }
}

export class Revwalk {
  constructor(private inner: NativeRevwalk) {}

  pushHead(): void {
    this.inner.pushHead();
  }

  push(oidStr: string): void {
    this.inner.push(oidStr);
  }

  hide(oidStr: string): void {
    this.inner.hide(oidStr);
  }

  next(): string | null {
    return this.inner.next();
  }
}

export class Worktree {
  constructor(private inner: NativeWorktree) {}

  name(): string | null {
    return this.inner.name();
  }

  path(): string {
    return this.inner.path();
  }

  validate(): void {
    this.inner.validate();
  }

  lock(reason?: string): void {
    this.inner.lock(reason ?? null);
  }

  unlock(): void {
    this.inner.unlock();
  }

  isLocked(): boolean {
    return this.inner.isLocked();
  }
}

export class Config {
  constructor(private inner: NativeConfig) {}

  getString(name: string): string {
    return this.inner.getString(name);
  }

  setString(name: string, value: string): void {
    this.inner.setString(name, value);
  }

  getBool(name: string): boolean {
    return this.inner.getBool(name);
  }

  setBool(name: string, value: boolean): void {
    this.inner.setBool(name, value);
  }

  remove(name: string): void {
    this.inner.remove(name);
  }
}
