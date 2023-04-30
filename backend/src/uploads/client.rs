use tokio::io::AsyncReadExt as _;

/// S3 client wrapper to expose semantic upload operations.
#[derive(Debug, Clone)]
pub struct Client {
    s3: aws_sdk_s3::Client,
    bucket_name: String,
}

impl Client {
    /// Construct S3 client wrapper.
    pub fn new(config: aws_sdk_s3::Config) -> Client {
        Client {
            s3: aws_sdk_s3::Client::from_conf(config),
            bucket_name: std::env::var("AWS_S3_BUCKET_NAME").unwrap(),
        }
    }

    pub fn url(&self, key: &str) -> String {
        format!(
            "https://{}.s3.{}.amazonaws.com/{key}",
            std::env::var("AWS_S3_BUCKET_NAME").unwrap(),
            std::env::var("AWS_REGION").unwrap(),
        )
    }

    /// Facilitate the upload of file to s3.
    pub async fn upload(
        &self,
        file: &actix_multipart::form::tempfile::TempFile,
        key_prefix: &str,
    ) -> crate::types::UploadedFile {
        let filename = file.file_name.as_deref().expect("TODO");
        let key = format!("{key_prefix}{filename}");
        let s3_url = self
            .put_object_from_file(file.file.path().to_str().unwrap(), &key)
            .await;
        crate::types::UploadedFile::new(filename, key, s3_url)
    }

    /// Real upload of file to S3
    async fn put_object_from_file(&self, local_path: &str, key: &str) -> String {
        let mut file = tokio::fs::File::open(local_path).await.unwrap();

        let size_estimate = file
            .metadata()
            .await
            .map(|md| md.len())
            .unwrap_or(1024)
            .try_into()
            .expect("file too big");

        let mut contents = Vec::with_capacity(size_estimate);
        file.read_to_end(&mut contents).await.unwrap();

        let _res = self
            .s3
            .put_object()
            .bucket(&self.bucket_name)
            .key(key)
            .body(aws_sdk_s3::primitives::ByteStream::from(contents))
            .send()
            .await
            .expect("Failed to put object");

        self.url(key)
    }

    /// Attempts to delete object from S3. Returns true if successful.
    pub async fn delete_file(&self, key: &str) -> bool {
        self.s3
            .delete_object()
            .bucket(&self.bucket_name)
            .key(key)
            .send()
            .await
            .is_ok()
    }
}
