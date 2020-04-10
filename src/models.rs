pub mod github {
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Debug, Serialize)]
    pub struct Asset {
        pub browser_download_url: String,
        pub name: String,
        pub label: String,
        pub size: u64,
        pub created_at: DateTime<Utc>,
        pub updated_at: DateTime<Utc>
    }

    #[derive(Deserialize, Debug, Serialize)]
    pub struct Release {
        pub html_url: String,
        pub id: u64,
        pub name: String,
        pub body: String,
        pub created_at: DateTime<Utc>,
        pub published_at: DateTime<Utc>,
        pub assets: Vec<Asset>
    }
}
