// @ts-ignore
import {
  Commit as NativeCommit,
  Reference as NativeReference,
  ReferenceType as NativeReferenceType,
  Repository as NativeRepository,
  RepositoryState as NativeRepositoryState,
  Signature as NativeSignature,
  Tree as NativeTree,
  Worktree as NativeWorktree,
} from "../index.js";

export { NativeReferenceType as ReferenceType, NativeRepositoryState as RepositoryState };

export class Signature {
  readonly name: string;
  readonly email: string;
  readonly timeSeconds: number;
  readonly timeOffsetMinutes: number;

  constructor(nativeSig: any) {
    this.name = nativeSig.name;
    this.email = nativeSig.email;
    this.timeSeconds = nativeSig.timeSeconds;
    this.timeOffsetMinutes = nativeSig.timeOffsetMinutes;
  }
}

export class Commit {
  private nativeCommit: any;

  constructor(nativeCommit: any) {
    this.nativeCommit = nativeCommit;
  }

  id(): string {
    return this.nativeCommit.id();
  }

  message(): string | null {
    return this.nativeCommit.message();
  }

  summary(): string | null {
    return this.nativeCommit.summary();
  }

  body(): string | null {
    return this.nativeCommit.body();
  }

  authorName(): string {
    return this.nativeCommit.authorName();
  }

  authorEmail(): string {
    return this.nativeCommit.authorEmail();
  }

  committerName(): string {
    return this.nativeCommit.committerName();
  }

  committerEmail(): string {
    return this.nativeCommit.committerEmail();
  }

  treeId(): string {
    return this.nativeCommit.treeId();
  }

  parentIds(): string[] {
    return this.nativeCommit.parentIds();
  }

  timeSeconds(): number {
    return this.nativeCommit.timeSeconds();
  }
}

export class Tree {
  private nativeTree: any;

  constructor(nativeTree: any) {
    this.nativeTree = nativeTree;
  }

  id(): string {
    return this.nativeTree.id();
  }

  len(): number {
    return this.nativeTree.len();
  }

  isEmpty(): boolean {
    return this.nativeTree.isEmpty();
  }
}

export class Worktree {
  private nativeWorktree: any;

  constructor(nativeWorktree: any) {
    this.nativeWorktree = nativeWorktree;
  }

  name(): string | null {
    return this.nativeWorktree.name();
  }

  path(): string {
    return this.nativeWorktree.path();
  }

  isLocked(): boolean {
    return this.nativeWorktree.isLocked();
  }
}

export class Reference {
  private nativeRef: any;

  constructor(nativeRef: any) {
    this.nativeRef = nativeRef;
  }

  name(): string | null {
    return this.nativeRef.name();
  }

  shorthand(): string | null {
    return this.nativeRef.shorthand();
  }

  target(): string | null {
    return this.nativeRef.target();
  }

  symbolicTarget(): string | null {
    return this.nativeRef.symbolicTarget();
  }

  kind(): any {
    return this.nativeRef.kind();
  }

  isBranch(): boolean {
    return this.nativeRef.isBranch();
  }

  isRemote(): boolean {
    return this.nativeRef.isRemote();
  }

  isTag(): boolean {
    return this.nativeRef.isTag();
  }

  isNote(): boolean {
    return this.nativeRef.isNote();
  }
}

export class Repository {
  private nativeRepo: any;

  private constructor(nativeRepo: any) {
    this.nativeRepo = nativeRepo;
  }

  static init(path: string): Repository {
    const repo = NativeRepository.init(path);
    return new Repository(repo);
  }

  static initBare(path: string): Repository {
    const repo = NativeRepository.initBare(path);
    return new Repository(repo);
  }

  static open(path: string): Repository {
    const repo = NativeRepository.open(path);
    return new Repository(repo);
  }

  static openBare(path: string): Repository {
    const repo = NativeRepository.openBare(path);
    return new Repository(repo);
  }

  static clone(url: string, path: string): Repository {
    const repo = NativeRepository.clone(url, path);
    return new Repository(repo);
  }

  static discover(path: string): Repository {
    const repo = NativeRepository.discover(path);
    return new Repository(repo);
  }

  path(): string {
    return this.nativeRepo.path();
  }

  workdir(): string | null {
    return this.nativeRepo.workdir();
  }

  isBare(): boolean {
    return this.nativeRepo.isBare();
  }

  isEmpty(): boolean {
    return this.nativeRepo.isEmpty();
  }

  isShallow(): boolean {
    return this.nativeRepo.isShallow();
  }

  isWorktree(): boolean {
    return this.nativeRepo.isWorktree();
  }

  state(): string {
    return this.nativeRepo.state();
  }

  stateEnum(): any {
    return this.nativeRepo.stateEnum();
  }

  headName(): string {
    return this.nativeRepo.headName();
  }

  headTarget(): string | null {
    return this.nativeRepo.headTarget();
  }

  addIgnoreRule(rules: string): void {
    this.nativeRepo.addIgnoreRule(rules);
  }

  clearIgnoreRules(): void {
    this.nativeRepo.clearIgnoreRules();
  }

  isPathIgnored(path: string): boolean {
    return this.nativeRepo.isPathIgnored(path);
  }

  remotes(): string[] {
    return this.nativeRepo.remotes();
  }

  createBranch(branchName: string, targetCommitId: string, force?: boolean): void {
    this.nativeRepo.createBranch(branchName, targetCommitId, force);
  }

  findCommit(oidHex: string): Commit {
    const commit = this.nativeRepo.findCommit(oidHex);
    return new Commit(commit);
  }

  findTree(oidHex: string): Tree {
    const tree = this.nativeRepo.findTree(oidHex);
    return new Tree(tree);
  }

  findReference(name: string): Reference {
    const ref = this.nativeRepo.findReference(name);
    return new Reference(ref);
  }

  refnameToId(name: string): string {
    return this.nativeRepo.refnameToId(name);
  }

  resolveReferenceFromShortName(refname: string): Reference {
    const ref = this.nativeRepo.resolveReferenceFromShortName(refname);
    return new Reference(ref);
  }

  createReference(name: string, targetOidHex: string, force: boolean, logMessage: string): Reference {
    const ref = this.nativeRepo.createReference(name, targetOidHex, force, logMessage);
    return new Reference(ref);
  }

  createSymbolicReference(name: string, target: string, force: boolean, logMessage: string): Reference {
    const ref = this.nativeRepo.createSymbolicReference(name, target, force, logMessage);
    return new Reference(ref);
  }

  referenceNames(): string[] {
    return this.nativeRepo.referenceNames();
  }

  signature(): Signature {
    const sig = this.nativeRepo.signature();
    return new Signature(sig);
  }

  commit(
    updateRef: string | null | undefined,
    authorName: string,
    authorEmail: string,
    committerName: string,
    committerEmail: string,
    message: string,
    treeId: string,
    parentIds: string[],
  ): string {
    return this.nativeRepo.commit(
      updateRef,
      authorName,
      authorEmail,
      committerName,
      committerEmail,
      message,
      treeId,
      parentIds,
    );
  }

  statusFile(path: string): string {
    return this.nativeRepo.statusFile(path);
  }

  worktrees(): string[] {
    return this.nativeRepo.worktrees();
  }

  findWorktree(name: string): Worktree {
    const wt = this.nativeRepo.findWorktree(name);
    return new Worktree(wt);
  }
}
