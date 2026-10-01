use git2::{
  Oid, ReferenceType as GitReferenceType, Repository as GitRepository,
  RepositoryState as GitRepositoryState, Signature as GitSignature,
};
use napi_derive::napi;
use std::path::Path;

#[napi]
pub enum RepositoryState {
  Clean,
  Merge,
  Revert,
  RevertSequence,
  CherryPick,
  CherryPickSequence,
  Bisect,
  Rebase,
  RebaseInteractive,
  RebaseMerge,
  ApplyMailbox,
  ApplyMailboxOrRebase,
}

impl From<GitRepositoryState> for RepositoryState {
  fn from(state: GitRepositoryState) -> Self {
    match state {
      GitRepositoryState::Clean => RepositoryState::Clean,
      GitRepositoryState::Merge => RepositoryState::Merge,
      GitRepositoryState::Revert => RepositoryState::Revert,
      GitRepositoryState::RevertSequence => RepositoryState::RevertSequence,
      GitRepositoryState::CherryPick => RepositoryState::CherryPick,
      GitRepositoryState::CherryPickSequence => RepositoryState::CherryPickSequence,
      GitRepositoryState::Bisect => RepositoryState::Bisect,
      GitRepositoryState::Rebase => RepositoryState::Rebase,
      GitRepositoryState::RebaseInteractive => RepositoryState::RebaseInteractive,
      GitRepositoryState::RebaseMerge => RepositoryState::RebaseMerge,
      GitRepositoryState::ApplyMailbox => RepositoryState::ApplyMailbox,
      GitRepositoryState::ApplyMailboxOrRebase => RepositoryState::ApplyMailboxOrRebase,
    }
  }
}

#[napi]
pub enum ReferenceType {
  Direct,
  Symbolic,
  Unknown,
}

impl From<Option<GitReferenceType>> for ReferenceType {
  fn from(kind: Option<GitReferenceType>) -> Self {
    match kind {
      Some(GitReferenceType::Direct) => ReferenceType::Direct,
      Some(GitReferenceType::Symbolic) => ReferenceType::Symbolic,
      _ => ReferenceType::Unknown,
    }
  }
}

#[napi]
pub struct Signature {
  pub name: String,
  pub email: String,
  pub time_seconds: i64,
  pub time_offset_minutes: i32,
}

#[napi]
pub struct Commit {
  id: String,
  message: Option<String>,
  summary: Option<String>,
  body: Option<String>,
  author_name: String,
  author_email: String,
  committer_name: String,
  committer_email: String,
  tree_id: String,
  parent_ids: Vec<String>,
  time_seconds: i64,
}

#[napi]
impl Commit {
  #[napi]
  pub fn id(&self) -> String {
    self.id.clone()
  }

  #[napi]
  pub fn message(&self) -> Option<String> {
    self.message.clone()
  }

  #[napi]
  pub fn summary(&self) -> Option<String> {
    self.summary.clone()
  }

  #[napi]
  pub fn body(&self) -> Option<String> {
    self.body.clone()
  }

  #[napi]
  pub fn author_name(&self) -> String {
    self.author_name.clone()
  }

  #[napi]
  pub fn author_email(&self) -> String {
    self.author_email.clone()
  }

  #[napi]
  pub fn committer_name(&self) -> String {
    self.committer_name.clone()
  }

  #[napi]
  pub fn committer_email(&self) -> String {
    self.committer_email.clone()
  }

  #[napi]
  pub fn tree_id(&self) -> String {
    self.tree_id.clone()
  }

  #[napi]
  pub fn parent_ids(&self) -> Vec<String> {
    self.parent_ids.clone()
  }

  #[napi]
  pub fn time_seconds(&self) -> i64 {
    self.time_seconds
  }
}

#[napi]
pub struct Tree {
  id: String,
  len: u32,
  is_empty: bool,
}

#[napi]
impl Tree {
  #[napi]
  pub fn id(&self) -> String {
    self.id.clone()
  }

  #[napi]
  pub fn len(&self) -> u32 {
    self.len
  }

  #[napi]
  pub fn is_empty(&self) -> bool {
    self.is_empty
  }
}

#[napi]
pub struct Worktree {
  name: Option<String>,
  path: String,
  is_locked: bool,
}

#[napi]
impl Worktree {
  #[napi]
  pub fn name(&self) -> Option<String> {
    self.name.clone()
  }

  #[napi]
  pub fn path(&self) -> String {
    self.path.clone()
  }

  #[napi]
  pub fn is_locked(&self) -> bool {
    self.is_locked
  }
}

#[napi]
pub struct Reference {
  name: Option<String>,
  shorthand: Option<String>,
  target: Option<String>,
  symbolic_target: Option<String>,
  kind: ReferenceType,
  is_branch: bool,
  is_remote: bool,
  is_tag: bool,
  is_note: bool,
}

#[napi]
impl Reference {
  #[napi]
  pub fn name(&self) -> Option<String> {
    self.name.clone()
  }

  #[napi]
  pub fn shorthand(&self) -> Option<String> {
    self.shorthand.clone()
  }

  #[napi]
  pub fn target(&self) -> Option<String> {
    self.target.clone()
  }

  #[napi]
  pub fn symbolic_target(&self) -> Option<String> {
    self.symbolic_target.clone()
  }

  #[napi]
  pub fn kind(&self) -> ReferenceType {
    match self.kind {
      ReferenceType::Direct => ReferenceType::Direct,
      ReferenceType::Symbolic => ReferenceType::Symbolic,
      ReferenceType::Unknown => ReferenceType::Unknown,
    }
  }

  #[napi]
  pub fn is_branch(&self) -> bool {
    self.is_branch
  }

  #[napi]
  pub fn is_remote(&self) -> bool {
    self.is_remote
  }

  #[napi]
  pub fn is_tag(&self) -> bool {
    self.is_tag
  }

  #[napi]
  pub fn is_note(&self) -> bool {
    self.is_note
  }
}

#[napi]
pub struct Repository {
  inner: GitRepository,
}

#[napi]
impl Repository {
  #[napi(factory)]
  pub fn init(path: String) -> napi::Result<Self> {
    let repo = GitRepository::init(path).map_err(|e| napi::Error::from_reason(e.to_string()))?;
    Ok(Self { inner: repo })
  }

  #[napi(factory)]
  pub fn init_bare(path: String) -> napi::Result<Self> {
    let repo = GitRepository::init_bare(path).map_err(|e| napi::Error::from_reason(e.to_string()))?;
    Ok(Self { inner: repo })
  }

  #[napi(factory)]
  pub fn open(path: String) -> napi::Result<Self> {
    let repo = GitRepository::open(path).map_err(|e| napi::Error::from_reason(e.to_string()))?;
    Ok(Self { inner: repo })
  }

  #[napi(factory)]
  pub fn open_bare(path: String) -> napi::Result<Self> {
    let repo = GitRepository::open_bare(path).map_err(|e| napi::Error::from_reason(e.to_string()))?;
    Ok(Self { inner: repo })
  }

  #[napi(factory)]
  pub fn clone(url: String, path: String) -> napi::Result<Self> {
    let repo = GitRepository::clone(&url, path).map_err(|e| napi::Error::from_reason(e.to_string()))?;
    Ok(Self { inner: repo })
  }

  #[napi(factory)]
  pub fn discover(path: String) -> napi::Result<Self> {
    let repo = GitRepository::discover(path).map_err(|e| napi::Error::from_reason(e.to_string()))?;
    Ok(Self { inner: repo })
  }

  #[napi]
  pub fn path(&self) -> String {
    self.inner.path().to_string_lossy().into_owned()
  }

  #[napi]
  pub fn workdir(&self) -> Option<String> {
    self.inner.workdir().map(|p| p.to_string_lossy().into_owned())
  }

  #[napi]
  pub fn is_bare(&self) -> bool {
    self.inner.is_bare()
  }

  #[napi]
  pub fn is_empty(&self) -> napi::Result<bool> {
    self.inner.is_empty().map_err(|e| napi::Error::from_reason(e.to_string()))
  }

  #[napi]
  pub fn is_shallow(&self) -> bool {
    self.inner.is_shallow()
  }

  #[napi]
  pub fn is_worktree(&self) -> bool {
    self.inner.is_worktree()
  }

  #[napi]
  pub fn state(&self) -> String {
    format!("{:?}", self.inner.state())
  }

  #[napi]
  pub fn state_enum(&self) -> RepositoryState {
    self.inner.state().into()
  }

  #[napi]
  pub fn head_name(&self) -> napi::Result<String> {
    let head = self.inner.head().map_err(|e| napi::Error::from_reason(e.to_string()))?;
    Ok(head.name().ok().unwrap_or("").to_string())
  }

  #[napi]
  pub fn head_target(&self) -> napi::Result<Option<String>> {
    let head = self.inner.head().map_err(|e| napi::Error::from_reason(e.to_string()))?;
    Ok(head.target().map(|oid| oid.to_string()))
  }

  #[napi]
  pub fn add_ignore_rule(&self, rules: String) -> napi::Result<()> {
    self.inner.add_ignore_rule(&rules).map_err(|e| napi::Error::from_reason(e.to_string()))
  }

  #[napi]
  pub fn clear_ignore_rules(&self) -> napi::Result<()> {
    self.inner.clear_ignore_rules().map_err(|e| napi::Error::from_reason(e.to_string()))
  }

  #[napi]
  pub fn is_path_ignored(&self, path: String) -> napi::Result<bool> {
    self.inner.is_path_ignored(path).map_err(|e| napi::Error::from_reason(e.to_string()))
  }

  #[napi]
  pub fn remotes(&self) -> napi::Result<Vec<String>> {
    let remotes = self.inner.remotes().map_err(|e| napi::Error::from_reason(e.to_string()))?;
    let mut list = Vec::new();
    for r in remotes.iter() {
      if let Ok(Some(name)) = r {
        list.push(name.to_string());
      }
    }
    Ok(list)
  }

  #[napi]
  pub fn create_branch(&self, branch_name: String, target_commit_id: String, force: Option<bool>) -> napi::Result<()> {
    let oid = Oid::from_str(&target_commit_id).map_err(|e| napi::Error::from_reason(e.to_string()))?;
    let commit = self.inner.find_commit(oid).map_err(|e| napi::Error::from_reason(e.to_string()))?;
    self.inner.branch(&branch_name, &commit, force.unwrap_or(false)).map_err(|e| napi::Error::from_reason(e.to_string()))?;
    Ok(())
  }

  #[napi]
  pub fn find_commit(&self, oid_hex: String) -> napi::Result<Commit> {
    let oid = Oid::from_str(&oid_hex).map_err(|e| napi::Error::from_reason(e.to_string()))?;
    let commit = self.inner.find_commit(oid).map_err(|e| napi::Error::from_reason(e.to_string()))?;
    let author = commit.author();
    let committer = commit.committer();
    let parent_ids = commit.parent_ids().map(|id| id.to_string()).collect();

    Ok(Commit {
      id: commit.id().to_string(),
      message: commit.message().ok().map(|s| s.to_string()),
      summary: commit.summary().ok().flatten().map(|s| s.to_string()),
      body: commit.body().ok().flatten().map(|s| s.to_string()),
      author_name: author.name().unwrap_or("").to_string(),
      author_email: author.email().unwrap_or("").to_string(),
      committer_name: committer.name().unwrap_or("").to_string(),
      committer_email: committer.email().unwrap_or("").to_string(),
      tree_id: commit.tree_id().to_string(),
      parent_ids,
      time_seconds: commit.time().seconds(),
    })
  }

  #[napi]
  pub fn find_tree(&self, oid_hex: String) -> napi::Result<Tree> {
    let oid = Oid::from_str(&oid_hex).map_err(|e| napi::Error::from_reason(e.to_string()))?;
    let tree = self.inner.find_tree(oid).map_err(|e| napi::Error::from_reason(e.to_string()))?;
    Ok(Tree {
      id: tree.id().to_string(),
      len: tree.len() as u32,
      is_empty: tree.is_empty(),
    })
  }

  #[napi]
  pub fn find_reference(&self, name: String) -> napi::Result<Reference> {
    let r = self.inner.find_reference(&name).map_err(|e| napi::Error::from_reason(e.to_string()))?;
    Ok(Reference {
      name: r.name().ok().map(String::from),
      shorthand: r.shorthand().ok().map(String::from),
      target: r.target().map(|oid| oid.to_string()),
      symbolic_target: r.symbolic_target().ok().flatten().map(String::from),
      kind: r.kind().into(),
      is_branch: r.is_branch(),
      is_remote: r.is_remote(),
      is_tag: r.is_tag(),
      is_note: r.is_note(),
    })
  }

  #[napi]
  pub fn refname_to_id(&self, name: String) -> napi::Result<String> {
    let oid = self.inner.refname_to_id(&name).map_err(|e| napi::Error::from_reason(e.to_string()))?;
    Ok(oid.to_string())
  }

  #[napi]
  pub fn resolve_reference_from_short_name(&self, refname: String) -> napi::Result<Reference> {
    let r = self.inner.resolve_reference_from_short_name(&refname).map_err(|e| napi::Error::from_reason(e.to_string()))?;
    Ok(Reference {
      name: r.name().ok().map(String::from),
      shorthand: r.shorthand().ok().map(String::from),
      target: r.target().map(|oid| oid.to_string()),
      symbolic_target: r.symbolic_target().ok().flatten().map(String::from),
      kind: r.kind().into(),
      is_branch: r.is_branch(),
      is_remote: r.is_remote(),
      is_tag: r.is_tag(),
      is_note: r.is_note(),
    })
  }

  #[napi]
  pub fn create_reference(&self, name: String, target_oid_hex: String, force: bool, log_message: String) -> napi::Result<Reference> {
    let oid = Oid::from_str(&target_oid_hex).map_err(|e| napi::Error::from_reason(e.to_string()))?;
    let r = self.inner.reference(&name, oid, force, &log_message).map_err(|e| napi::Error::from_reason(e.to_string()))?;
    Ok(Reference {
      name: r.name().ok().map(String::from),
      shorthand: r.shorthand().ok().map(String::from),
      target: r.target().map(|oid| oid.to_string()),
      symbolic_target: r.symbolic_target().ok().flatten().map(String::from),
      kind: r.kind().into(),
      is_branch: r.is_branch(),
      is_remote: r.is_remote(),
      is_tag: r.is_tag(),
      is_note: r.is_note(),
    })
  }

  #[napi]
  pub fn create_symbolic_reference(&self, name: String, target: String, force: bool, log_message: String) -> napi::Result<Reference> {
    let r = self.inner.reference_symbolic(&name, &target, force, &log_message).map_err(|e| napi::Error::from_reason(e.to_string()))?;
    Ok(Reference {
      name: r.name().ok().map(String::from),
      shorthand: r.shorthand().ok().map(String::from),
      target: r.target().map(|oid| oid.to_string()),
      symbolic_target: r.symbolic_target().ok().flatten().map(String::from),
      kind: r.kind().into(),
      is_branch: r.is_branch(),
      is_remote: r.is_remote(),
      is_tag: r.is_tag(),
      is_note: r.is_note(),
    })
  }

  #[napi]
  pub fn reference_names(&self) -> napi::Result<Vec<String>> {
    let refs = self.inner.references().map_err(|e| napi::Error::from_reason(e.to_string()))?;
    let mut names = Vec::new();
    for r in refs {
      if let Ok(r) = r {
        if let Ok(name) = r.name() {
          names.push(name.to_string());
        }
      }
    }
    Ok(names)
  }

  #[napi]
  pub fn signature(&self) -> napi::Result<Signature> {
    let sig = self.inner.signature().map_err(|e| napi::Error::from_reason(e.to_string()))?;
    let time = sig.when();
    Ok(Signature {
      name: sig.name().unwrap_or("").to_string(),
      email: sig.email().unwrap_or("").to_string(),
      time_seconds: time.seconds(),
      time_offset_minutes: time.offset_minutes(),
    })
  }

  #[napi]
  pub fn commit(
    &self,
    update_ref: Option<String>,
    author_name: String,
    author_email: String,
    committer_name: String,
    committer_email: String,
    message: String,
    tree_id: String,
    parent_ids: Vec<String>,
  ) -> napi::Result<String> {
    let author = GitSignature::now(&author_name, &author_email).map_err(|e| napi::Error::from_reason(e.to_string()))?;
    let committer = GitSignature::now(&committer_name, &committer_email).map_err(|e| napi::Error::from_reason(e.to_string()))?;
    let tree_oid = Oid::from_str(&tree_id).map_err(|e| napi::Error::from_reason(e.to_string()))?;
    let tree = self.inner.find_tree(tree_oid).map_err(|e| napi::Error::from_reason(e.to_string()))?;

    let mut parents = Vec::new();
    for pid in &parent_ids {
      let p_oid = Oid::from_str(pid).map_err(|e| napi::Error::from_reason(e.to_string()))?;
      let parent = self.inner.find_commit(p_oid).map_err(|e| napi::Error::from_reason(e.to_string()))?;
      parents.push(parent);
    }
    let parent_refs: Vec<&git2::Commit> = parents.iter().collect();

    let oid = self
      .inner
      .commit(
        update_ref.as_deref(),
        &author,
        &committer,
        &message,
        &tree,
        &parent_refs,
      )
      .map_err(|e| napi::Error::from_reason(e.to_string()))?;

    Ok(oid.to_string())
  }

  #[napi]
  pub fn status_file(&self, path: String) -> napi::Result<String> {
    let status = self.inner.status_file(Path::new(&path)).map_err(|e| napi::Error::from_reason(e.to_string()))?;
    Ok(format!("{:?}", status))
  }

  #[napi]
  pub fn worktrees(&self) -> napi::Result<Vec<String>> {
    let wt = self.inner.worktrees().map_err(|e| napi::Error::from_reason(e.to_string()))?;
    let mut list = Vec::new();
    for r in wt.iter() {
      if let Ok(Some(name)) = r {
        list.push(name.to_string());
      }
    }
    Ok(list)
  }

  #[napi]
  pub fn find_worktree(&self, name: String) -> napi::Result<Worktree> {
    let wt = self.inner.find_worktree(&name).map_err(|e| napi::Error::from_reason(e.to_string()))?;
    let wt_name = wt.name().ok().flatten().map(|s| s.to_string());
    let wt_path = wt.path().to_string_lossy().into_owned();
    let is_locked = wt.is_locked().map(|s| match s {
      git2::WorktreeLockStatus::Locked(_) => true,
      git2::WorktreeLockStatus::Unlocked => false,
    }).unwrap_or(false);

    Ok(Worktree {
      name: wt_name,
      path: wt_path,
      is_locked,
    })
  }
}
