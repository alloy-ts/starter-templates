use git2::{Oid, Repository as GitRepository, Signature};
use napi_derive::napi;
use std::path::Path;

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
  pub fn head_name(&self) -> napi::Result<String> {
    let head = self.inner.head().map_err(|e| napi::Error::from_reason(e.to_string()))?;
    Ok(head.name().unwrap_or("").to_string())
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
    let author = Signature::now(&author_name, &author_email).map_err(|e| napi::Error::from_reason(e.to_string()))?;
    let committer = Signature::now(&committer_name, &committer_email).map_err(|e| napi::Error::from_reason(e.to_string()))?;
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
}
