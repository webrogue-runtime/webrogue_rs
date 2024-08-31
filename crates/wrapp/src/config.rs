#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Config {
    filesystem: Option<FilesystemConfig>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct FilesystemConfig {
    dirs: Vec<FilesystemDirConfig>,
    resources: Vec<FilesystemResourceConfig>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct FilesystemDirConfig {
    #[serde(rename = "type")]
    ty: String,
    name: String,
    path: String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct FilesystemResourceConfig {
    real_path: String,
    dirs: Vec<FilesystemResourceDirConfig>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct FilesystemResourceDirConfig {
    name: String,
    path: String,
}
