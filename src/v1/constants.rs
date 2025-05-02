use serde::{Deserialize, Serialize};

pub const API_URL_BASE: &str = "https://api.mistral.ai/v1";

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum Model {
    #[serde(rename = "ministral-3b-latest", alias = "ministral-3b-2410")]
    Ministral3b,
    #[serde(rename = "ministral-8b-latest", alias = "ministral-8b-2410")]
    Ministral8b,
    #[serde(rename = "mistral-small-latest", alias = "mistral-small-2502")]
    MistralSmall,
    #[serde(rename = "open-mistral-nemo", alias = "mistral-nemo-2502")]
    OpenMistralNemo,
    #[serde(rename = "codestral-latest", alias = "codestral-2501")]
    Codestral,
    #[serde(rename = "mistral-saba-latest", alias = "mistral-saba-2502")]
    MistralSaba,
    #[serde(rename = "mistral-large-latest", alias = "mistral-large-2411")]
    MistralLarge,
    #[serde(rename = "pixtral-large-latest", alias = "pixtral-large-2411")]
    PixtralLarge,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum EmbedModel {
    #[serde(rename = "mistral-embed")]
    MistralEmbed,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum OcrModel {
    #[serde(rename = "mistral-ocr-latest", alias = "mistral-ocr-2503")]
    MistralEmbed,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum ModerationModel {
    #[serde(
        rename = "mistral-moderation-latest",
        alias = "mistral-moderation-2411"
    )]
    MistralModeration,
}
