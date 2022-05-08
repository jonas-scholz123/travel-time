use anyhow::{Ok, Result};
use rusoto_core::Region;
use rusoto_s3::{Bucket, GetObjectRequest, ListObjectsRequest, S3Client, S3};
use tokio::{
    fs::File,
    io::{copy, AsyncReadExt, AsyncWriteExt},
};

pub struct NationalRailS3 {}

impl NationalRailS3 {
    pub async fn get_timetable_data() -> Result<()> {
        let region = Region::EuWest1;
        let client = S3Client::new(region);
        let bucket_name = "darwin.xmltimetable".to_string();
        let request = ListObjectsRequest {
            bucket: bucket_name.clone(),
            prefix: Some("PPTimetable/".to_string()),
            ..Default::default()
        };
        let objects = client.list_objects(request).await?.contents.unwrap();

        let timetable_key = objects
            .iter()
            .filter(|obj| obj.key.as_ref().unwrap().ends_with("_v8.xml.gz"))
            .max_by_key(|obj| obj.last_modified.as_ref().unwrap())
            .unwrap()
            .key
            .as_ref()
            .unwrap()
            .to_string();

        println!("{:#?}", timetable_key);

        let request = GetObjectRequest {
            bucket: bucket_name.clone(),
            key: timetable_key,
            ..Default::default()
        };

        let timetable_file = client.get_object(request).await?;

        let mut s3_file = timetable_file.body.unwrap().into_async_read();

        let mut local_file = File::create("./data/timetable.xml.gz").await?;

        copy(&mut s3_file, &mut local_file).await?;
        Ok(())
    }
}
