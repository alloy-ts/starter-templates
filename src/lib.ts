// @ts-ignore
import { Repository as NativeRepository } from "../index.js";

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
}
