use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct FileUploadDO {
    pub bundle_id: String,
    pub version_id: String,
    pub md5: String,
}
