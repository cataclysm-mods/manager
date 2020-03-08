use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Deserialize, Debug, Serialize)]
pub struct Asset {
    pub browser_download_url: String,
    pub name: String,
    pub label: String,
    pub size: u64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl fmt::Display for Asset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let updated_at = self.updated_at.format("%a %b %e %T %Y");
        write!(f, "Asset: {} {} {}", self.label, updated_at, self.browser_download_url)
    }
}


#[derive(Deserialize, Debug, Serialize)]
pub struct Release {
    pub html_url: String,
    pub id: u64,
    pub name: String,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub published_at: DateTime<Utc>,
    pub assets: Vec<Asset>,
}

impl fmt::Display for Release {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let published_at = self.published_at.format("%a %b %e %T %Y");
        write!(f, "Release: {} {} {}", self.name, published_at, self.html_url)
    }
}
