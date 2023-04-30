#[derive(Debug, serde::Serialize, Clone)]
pub struct UploadedFile {
    filename: String,
    s3_key: String,
    pub s3_url: String,
}

impl UploadedFile {
    /// Construct new uploaded file info container.
    pub fn new(
        filename: impl Into<String>,
        s3_key: impl Into<String>,
        s3_url: impl Into<String>,
    ) -> Self {
        Self {
            filename: filename.into(),
            s3_key: s3_key.into(),
            s3_url: s3_url.into(),
        }
    }
}
