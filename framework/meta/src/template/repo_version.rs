pub enum RepoVersion {
    Master,
    Tag(String),
}

impl RepoVersion {
    pub fn url(&self) -> String {
        match self {
            RepoVersion::Master => {
                "https://github.com/Battula-anil-app-could/dx-sdk-rs-prechecker/archive/refs/heads/master.zip".to_string()
            },
            RepoVersion::Tag(tag) => {
                format!("https://github.com/Battula-anil-app-could/dx-sdk-rs-prechecker/archive/refs/tags/v{tag}.zip")
            },
        }
    }

    pub fn temp_dir_name(&self) -> String {
        match self {
            RepoVersion::Master => "dx-sdk-rs-prechecker-master".to_string(),
            RepoVersion::Tag(tag) => {
                format!("dx-sdk-rs-prechecker-{tag}")
            },
        }
    }
}
