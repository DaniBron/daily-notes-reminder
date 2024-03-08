use std::io::BufWriter;
use std::{fs::File, io::Write};

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{primitives::ByteStream, Client};
use aws_types::SdkConfig;

const BUCKET: &str = "daily-notes-reminder";

pub struct S3Manager {}

impl S3Manager {
    pub fn new() -> Self {
        S3Manager {}
    }

    pub async fn download_file(&self, key: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("Getting file: {:?}", key);

        let config: SdkConfig = self.config_builder().await?;

        let mut data: ByteStream = self.get_byte_stream(key, &config).await?;

        self.save_file(&mut data, key).await?;

        Ok(())
    }

    async fn config_builder(&self) -> Result<SdkConfig, Box<dyn std::error::Error>> {
        let region_provider = RegionProviderChain::default_provider();

        let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
            .region(region_provider)
            .load()
            .await;

        Ok(config)
    }

    async fn get_byte_stream(&self, key: &str, config: &SdkConfig) -> Result<ByteStream, Box<dyn std::error::Error>> {
        let client = Client::new(&config);

        let object = client.get_object().bucket(BUCKET).key(key).send().await?;

        Ok(object.body)
    }

    async fn save_file(&self, data: &mut ByteStream, key: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file: File = File::create(key)?;

        let mut buf_writer = BufWriter::new(file);

        while let Some(chunk) = data.try_next().await? {
            buf_writer.write(&chunk)?;
        }

        buf_writer.flush()?;

        Ok(())
    }
}
