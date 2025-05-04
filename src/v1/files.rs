use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum FilePurpose {
    #[serde(rename = "fine-tune")]
    FineTune,
    #[serde(rename = "batch")]
    Batch,
    #[serde(rename = "ocr")]
    Ocr,
    #[serde(rename = "code_interpreter")]
    CodeInterpreter,
    #[serde(rename = "image_generation")]
    ImageGeneration,
}
impl fmt::Display for FilePurpose {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FilePurpose::FineTune => write!(f, "fine-tune"),
            FilePurpose::Batch => write!(f, "batch"),
            FilePurpose::Ocr => write!(f, "ocr"),
            FilePurpose::CodeInterpreter => write!(f, "code_interpreter"),
            FilePurpose::ImageGeneration => write!(f, "image_generation"),
        }
    }
}

impl From<FilePurpose> for Cow<'static, str> {
    fn from(purpose: FilePurpose) -> Self {
        Cow::Owned(purpose.to_string())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum SampleType {
    #[serde(rename = "pretrain")]
    Pretrain,
    #[serde(rename = "instruct")]
    Instruct,
    #[serde(rename = "batch_request")]
    BatchRequest,
    #[serde(rename = "batch_result")]
    BatchResult,
    #[serde(rename = "batch_error")]
    BatchError,
    #[serde(rename = "ocr_input")]
    OcrInput,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Source {
    #[serde(rename = "upload")]
    Upload,
    #[serde(rename = "repository")]
    Repository,
    #[serde(rename = "mistral")]
    Mistral,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UploadRequest {
    pub file: String,
    pub purpose: FilePurpose,
}

impl UploadRequest {
    pub fn new(file: &str, purpose: FilePurpose) -> Self {
        Self {
            file: file.to_owned(),
            purpose,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UploadResponse {
    pub id: String,
    pub object: String,
    pub bytes: u64,
    pub created_at: u32,
    pub filename: String,
    pub purpose: FilePurpose,
    pub sample_type: SampleType,
    pub num_lines: u32,
    pub source: Source,
}
