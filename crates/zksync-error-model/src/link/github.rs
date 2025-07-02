use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd)]
pub struct BranchName(pub String);

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd)]
pub struct CommitHash(pub String);

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd)]
pub struct GithubLink {
    pub repo: String,
    pub path: String,
    #[serde(flatten)]
    pub reference: ReferenceType,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd)]
#[serde(untagged)]
pub enum ReferenceType {
    Branch { branch: BranchName },
    Commit { commit: CommitHash },
}

impl Default for ReferenceType {
    fn default() -> Self {
        ReferenceType::Branch {
            branch: default_branch(),
        }
    }
}

fn default_branch() -> BranchName {
    BranchName("main".to_string())
}

impl GithubLink {
    pub fn loose_eq(&self, other: &Self) -> bool {
        self.repo == other.repo && self.path == other.path
    }

    pub fn new_with_branch(repo: String, path: String, branch: BranchName) -> Self {
        Self {
            repo,
            path,
            reference: ReferenceType::Branch { branch },
        }
    }

    pub fn new_with_commit(repo: String, path: String, commit: CommitHash) -> Self {
        Self {
            repo,
            path,
            reference: ReferenceType::Commit { commit },
        }
    }

    pub fn to_url(&self) -> String {
        let Self {
            repo,
            path,
            reference,
        } = self;
        match reference {
            ReferenceType::Branch { branch } => {
                format!(
                    "https://raw.githubusercontent.com/{repo}/refs/heads/{}/{path}",
                    branch.0
                )
            }
            ReferenceType::Commit { commit } => {
                format!(
                    "https://raw.githubusercontent.com/{repo}/{}/{path}",
                    commit.0
                )
            }
        }
    }
}

impl fmt::Display for GithubLink {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_url())
    }
}

impl fmt::Display for BranchName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for CommitHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
