use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PkgInfo {
    pub pkg_name: String,
    pub title: String,
    pub url: String,
}