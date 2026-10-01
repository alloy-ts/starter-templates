import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import { expect, test } from "vite-plus/test";
import { ReferenceType, Repository, RepositoryState } from "./lib.ts";

test("Repository.init creates a new repository", () => {
  const tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), "git2-test-init-"));
  try {
    const repo = Repository.init(tmpDir);
    expect(repo.isBare()).toBe(false);
    expect(repo.isEmpty()).toBe(true);
    expect(repo.path()).toContain(".git");
    expect(repo.workdir()).toBeTruthy();
    expect(repo.stateEnum()).toBe(RepositoryState.Clean);
  } finally {
    fs.rmSync(tmpDir, { recursive: true, force: true });
  }
});

test("Repository.initBare creates a bare repository", () => {
  const tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), "git2-test-bare-"));
  try {
    const repo = Repository.initBare(tmpDir);
    expect(repo.isBare()).toBe(true);
    expect(repo.isEmpty()).toBe(true);
    expect(repo.workdir()).toBeNull();
  } finally {
    fs.rmSync(tmpDir, { recursive: true, force: true });
  }
});

test("Repository.open opens an existing repository", () => {
  const tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), "git2-test-open-"));
  try {
    Repository.init(tmpDir);
    const repo = Repository.open(tmpDir);
    expect(repo.isBare()).toBe(false);
    expect(repo.isEmpty()).toBe(true);
  } finally {
    fs.rmSync(tmpDir, { recursive: true, force: true });
  }
});

test("Repository.discover finds repository path", () => {
  const tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), "git2-test-discover-"));
  const subDir = path.join(tmpDir, "sub", "dir");
  fs.mkdirSync(subDir, { recursive: true });
  try {
    Repository.init(tmpDir);
    const repo = Repository.discover(subDir);
    expect(repo.path()).toContain(".git");
  } finally {
    fs.rmSync(tmpDir, { recursive: true, force: true });
  }
});

test("Repository ignore rules and status", () => {
  const tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), "git2-test-ignore-"));
  try {
    const repo = Repository.init(tmpDir);
    repo.addIgnoreRule("*.log");
    expect(repo.isPathIgnored("app.log")).toBe(true);
    expect(repo.isPathIgnored("app.txt")).toBe(false);
    repo.clearIgnoreRules();
  } finally {
    fs.rmSync(tmpDir, { recursive: true, force: true });
  }
});

test("Repository references operations", () => {
  const tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), "git2-test-refs-"));
  try {
    const repo = Repository.init(tmpDir);
    const symRef = repo.createSymbolicReference("HEAD", "refs/heads/main", true, "set head to main");
    expect(symRef.name()).toBe("HEAD");
    expect(symRef.symbolicTarget()).toBe("refs/heads/main");
    expect(symRef.kind()).toBe(ReferenceType.Symbolic);

    const names = repo.referenceNames();
    expect(Array.isArray(names)).toBe(true);
  } finally {
    fs.rmSync(tmpDir, { recursive: true, force: true });
  }
});
