use s3::{creds::Credentials, Bucket, Region};
use std::{env, error::Error, str::FromStr, sync::Arc};

pub struct ApplicantFilesBucket {
    bucket: Bucket,
}

pub struct S3Buckets {
    applicant_files: ApplicantFilesBucket,
}

impl S3Buckets {
    pub fn applicant_files(&self) -> &ApplicantFilesBucket {
        &self.applicant_files
    }
}

pub enum BucketManager {
    Initialized(S3Buckets),
    Error(Arc<dyn Error + Send + Sync>),
    Uninitialized,
}

impl BucketManager {
    pub fn new() -> BucketManager {
        BucketManager::Uninitialized
    }

    fn initialize_buckets() -> Result<S3Buckets, Box<dyn Error + Sync + Send>> {
        let credentials = Credentials::from_env_specific(
            Some("FELIX_AWS_ACCESS_KEY_ID"),
            Some("FELIX_AWS_SECRET_ACCESS_KEY"),
            None,
            None,
        )?;

        let applicant_files_bucket_name = env::var("S3_APPLICANT_FILES_BUCKET_NAME")?;
        let applicant_files_bucket_region =
            Region::from_str(&env::var("S3_APPLICANT_FILES_REGION")?)?;

        let applicant_files_bucket = Bucket::new(
            &applicant_files_bucket_name,
            applicant_files_bucket_region,
            credentials,
        )?;

        let applicant_files = ApplicantFilesBucket {
            bucket: applicant_files_bucket,
        };

        Ok(S3Buckets { applicant_files })
    }

    pub fn get_buckets(&mut self) -> Result<&S3Buckets, Arc<dyn Error + Send + Sync>> {
        use BucketManager::*;
        match self {
            Initialized(v) => Ok(v),
            Error(e) => Err(e.clone()),
            Uninitialized => {
                match BucketManager::initialize_buckets() {
                    Ok(v) => *self = Initialized(v),
                    Err(e) => *self = Error(Arc::from(e)),
                }
                self.get_buckets()
            }
        }
    }
}

impl ApplicantFilesBucket {
    pub async fn add_applicant_cv(
        &self,
        cv_file: &[u8],
        applicant_id: i32,
    ) -> anyhow::Result<String> {
        let object_path = format!("cv-{}", applicant_id);
        self.bucket.put_object(object_path.clone(), cv_file).await?;

        Ok(object_path)
    }
}
