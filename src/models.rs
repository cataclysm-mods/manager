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

        // Set an empty list of assets for cases where
        // the Github API returns a relase without an `assets` property
        #[serde(default)]
        pub assets: Vec<Asset>
    }

    impl Default for Release {
        fn default() -> Self {
            Release { assets: vec![], ..Default::default() }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::github::*;

    #[test]
    fn test_release_model_defaults() -> Result<(), serde_json::Error> {
        let json = r#"
        {
            "id": 1234567890,
            "name": "Name",
            "body": "Body",
            "html_url": "https://github.com/some/path/to/an/object",
            "created_at": "2021-01-19T01:15:00Z",
            "published_at": "2021-01-19T01:15:00Z"
        }
        "#;

        let release: Release = serde_json::from_str(json)?;

        assert_eq!(release.assets.len(), 0);

        Ok(())
    }

    #[test]
    fn test_release_model_with_empty_assets() -> Result<(), serde_json::Error> {
        let json = r#"
        {
            "id": 1234567890,
            "name": "Name",
            "body": "Body",
            "html_url": "https://github.com/some/path/to/an/object",
            "created_at": "2021-01-19T01:15:00Z",
            "published_at": "2021-01-19T01:15:00Z",
            "assets": []
        }
        "#;

        let release: Release = serde_json::from_str(json)?;

        assert_eq!(release.assets.len(), 0);

        Ok(())
    }

    #[test]
    fn test_release_model_with_assets() -> Result<(), serde_json::Error> {
        let json = r#"
        {
            "id": 1234567890,
            "name": "Name",
            "body": "Body",
            "html_url": "https://github.com/some/path/to/an/object",
            "created_at": "2021-01-19T01:15:00Z",
            "published_at": "2021-01-19T01:15:00Z",
            "assets": [{
                "browser_download_url": "https://github.com/some/path/to/an/object",
                "name": "MyAsset_01.zip",
                "label": "My Asset",
                "size": 2048,
                "created_at": "2021-01-19T01:15:00Z",
                "updated_at": "2021-01-19T01:15:00Z"
            }]
        }
        "#;

        let release: Release = serde_json::from_str(json)?;

        assert_eq!(release.assets.len(), 1);

        Ok(())
    }
}