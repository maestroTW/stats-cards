use axum::http::{HeaderMap, HeaderValue};
use lazy_static::lazy_static;
use reqwest::{Client, Error, header::AUTHORIZATION};
use serde::{Deserialize, Serialize};

use crate::{data::config::CONFIG, pub_struct};

lazy_static! {
    static ref REQ_CLIENT: Client = Client::new();
}

#[derive(Debug, Deserialize, Serialize)]
pub enum PipelineTag {
    #[serde(rename = "text-classification")]
    TextClassification,
    #[serde(rename = "token-classification")]
    TokenClassification,
    #[serde(rename = "table-question-answering")]
    TableQuestionAnswering,
    #[serde(rename = "question-answering")]
    QuestionAnswering,
    #[serde(rename = "zero-shot-classification")]
    ZeroShotClassification,
    #[serde(rename = "translation")]
    Translation,
    #[serde(rename = "summarization")]
    Summarization,
    #[serde(rename = "feature-extraction")]
    FeatureExtraction,
    #[serde(rename = "text-generation")]
    TextGeneration,
    #[serde(rename = "text2text-generation")]
    Text2TextGeneration,
    #[serde(rename = "fill-mask")]
    FillMask,
    #[serde(rename = "sentence-similarity")]
    SentenceSimilarity,
    #[serde(rename = "text-to-speech")]
    TexttoSpeech,
    #[serde(rename = "text-to-audio")]
    TexttoAudio,
    #[serde(rename = "automatic-speech-recognition")]
    AutomaticSpeechRecognition,
    #[serde(rename = "audio-to-audio")]
    AudiotoAudio,
    #[serde(rename = "audio-classification")]
    AudioClassification,
    #[serde(rename = "audio-text-to-text")]
    AudioTexttoText,
    #[serde(rename = "voice-activity-detection")]
    VoiceActivityDetection,
    #[serde(rename = "depth-estimation")]
    DepthEstimation,
    #[serde(rename = "image-classification")]
    ImageClassification,
    #[serde(rename = "object-detection")]
    ObjectDetection,
    #[serde(rename = "image-segmentation")]
    ImageSegmentation,
    #[serde(rename = "text-to-image")]
    TexttoImage,
    #[serde(rename = "image-to-text")]
    ImagetoText,
    #[serde(rename = "image-to-image")]
    ImagetoImage,
    #[serde(rename = "image-to-video")]
    ImagetoVideo,
    #[serde(rename = "unconditional-image-generation")]
    UnconditionalImageGeneration,
    #[serde(rename = "video-classification")]
    VideoClassification,
    #[serde(rename = "reinforcement-learning")]
    ReinforcementLearning,
    #[serde(rename = "robotics")]
    Robotics,
    #[serde(rename = "tabular-classification")]
    TabularClassification,
    #[serde(rename = "tabular-regression")]
    TabularRegression,
    #[serde(rename = "tabular-to-text")]
    TabulartoText,
    #[serde(rename = "table-to-text")]
    TabletoText,
    #[serde(rename = "multiple-choice")]
    MultipleChoice,
    #[serde(rename = "text-ranking")]
    TextRanking,
    #[serde(rename = "text-retrieval")]
    TextRetrieval,
    #[serde(rename = "time-series-forecasting")]
    TimeSeriesForecasting,
    #[serde(rename = "text-to-video")]
    TexttoVideo,
    #[serde(rename = "image-text-to-text")]
    ImageTexttoText,
    #[serde(rename = "visual-question-answering")]
    VisualQuestionAnswering,
    #[serde(rename = "document-question-answering")]
    DocumentQuestionAnswering,
    #[serde(rename = "zero-shot-image-classification")]
    ZeroShotImageClassification,
    #[serde(rename = "graph-ml")]
    GraphMachineLearning,
    #[serde(rename = "mask-generation")]
    MaskGeneration,
    #[serde(rename = "zero-shot-object-detection")]
    ZeroShotObjectDetection,
    #[serde(rename = "text-to-3d")]
    Textto3D,
    #[serde(rename = "image-to-3d")]
    Imageto3D,
    #[serde(rename = "image-feature-extraction")]
    ImageFeatureExtraction,
    #[serde(rename = "video-text-to-text")]
    VideoTexttoText,
    #[serde(rename = "keypoint-detection")]
    KeypointDetection,
    #[serde(rename = "visual-document-retrieval")]
    VisualDocumentRetrieval,
    #[serde(rename = "any-to-any")]
    AnytoAny,
    #[serde(rename = "other")]
    Other,
}

impl PipelineTag {
    pub fn to_string(&self) -> String {
        match self {
            PipelineTag::TextClassification => "Text Classification".to_string(),
            PipelineTag::TokenClassification => "Token Classification".to_string(),
            PipelineTag::TableQuestionAnswering => "Table Question Answering".to_string(),
            PipelineTag::QuestionAnswering => "Question Answering".to_string(),
            PipelineTag::ZeroShotClassification => "Zero-Shot Classification".to_string(),
            PipelineTag::Translation => "Translation".to_string(),
            PipelineTag::Summarization => "Summarization".to_string(),
            PipelineTag::FeatureExtraction => "Feature Extraction".to_string(),
            PipelineTag::TextGeneration => "Text Generation".to_string(),
            PipelineTag::Text2TextGeneration => "Text2Text Generation".to_string(),
            PipelineTag::FillMask => "Fill-Mask".to_string(),
            PipelineTag::SentenceSimilarity => "Sentence Similarity".to_string(),
            PipelineTag::TexttoSpeech => "Text-to-Speech".to_string(),
            PipelineTag::TexttoAudio => "Text-to-Audio".to_string(),
            PipelineTag::AutomaticSpeechRecognition => "Automatic Speech Recognition".to_string(),
            PipelineTag::AudiotoAudio => "Audio-to-Audio".to_string(),
            PipelineTag::AudioClassification => "Audio Classification".to_string(),
            PipelineTag::AudioTexttoText => "Audio-Text-to-Text".to_string(),
            PipelineTag::VoiceActivityDetection => "Voice Activity Detection".to_string(),
            PipelineTag::DepthEstimation => "Depth Estimation".to_string(),
            PipelineTag::ImageClassification => "Image Classification".to_string(),
            PipelineTag::ObjectDetection => "Object Detection".to_string(),
            PipelineTag::ImageSegmentation => "Image Segmentation".to_string(),
            PipelineTag::TexttoImage => "Text-to-Image".to_string(),
            PipelineTag::ImagetoText => "Image-to-Text".to_string(),
            PipelineTag::ImagetoImage => "Image-to-Image".to_string(),
            PipelineTag::ImagetoVideo => "Image-to-Video".to_string(),
            PipelineTag::UnconditionalImageGeneration => {
                "Unconditional Image Generation".to_string()
            }
            PipelineTag::VideoClassification => "Video Classification".to_string(),
            PipelineTag::ReinforcementLearning => "Reinforcement Learning".to_string(),
            PipelineTag::Robotics => "Robotics".to_string(),
            PipelineTag::TabularClassification => "Tabular Classification".to_string(),
            PipelineTag::TabularRegression => "Tabular Regression".to_string(),
            PipelineTag::TabulartoText => "Tabular to Text".to_string(),
            PipelineTag::TabletoText => "Table to Text".to_string(),
            PipelineTag::MultipleChoice => "Multiple Choice".to_string(),
            PipelineTag::TextRanking => "Text Ranking".to_string(),
            PipelineTag::TextRetrieval => "Text Retrieval".to_string(),
            PipelineTag::TimeSeriesForecasting => "Time Series Forecasting".to_string(),
            PipelineTag::TexttoVideo => "Text-to-Video".to_string(),
            PipelineTag::ImageTexttoText => "Image-Text-to-Text".to_string(),
            PipelineTag::VisualQuestionAnswering => "Visual Question Answering".to_string(),
            PipelineTag::DocumentQuestionAnswering => "Document Question Answering".to_string(),
            PipelineTag::ZeroShotImageClassification => {
                "Zero-Shot Image Classification".to_string()
            }
            PipelineTag::GraphMachineLearning => "Graph Machine Learning".to_string(),
            PipelineTag::MaskGeneration => "Mask Generation".to_string(),
            PipelineTag::ZeroShotObjectDetection => "Zero-Shot Object Detection".to_string(),
            PipelineTag::Textto3D => "Text-to-3D".to_string(),
            PipelineTag::Imageto3D => "Image-to-3D".to_string(),
            PipelineTag::ImageFeatureExtraction => "Image Feature Extraction".to_string(),
            PipelineTag::VideoTexttoText => "Video-Text-to-Text".to_string(),
            PipelineTag::KeypointDetection => "Keypoint Detection".to_string(),
            PipelineTag::VisualDocumentRetrieval => "Visual Document Retrieval".to_string(),
            PipelineTag::AnytoAny => "Any-to-Any".to_string(),
            PipelineTag::Other => "Other".to_string(),
        }
    }
}

pub_struct! { CardData {
    license: Option<String>,
    tags: Option<Vec<String>>,
}}

pub_struct! { ModelConfig {
    model_type: Option<String>,
}}

#[derive(Debug, Deserialize, Serialize)]
pub struct Model {
    // without same useless fields
    pub _id: String,
    pub id: String,
    pub private: bool,
    pub pipeline_tag: Option<PipelineTag>,
    // e.g. transformers
    pub library_name: Option<String>,
    pub tags: Vec<String>,
    pub downloads: u32,
    pub likes: u32,
    #[serde(rename = "modelId")]
    pub model_id: String,
    pub author: String,
    #[serde(rename = "lastModified")]
    pub last_modified: String,
    pub gated: bool,
    pub disabled: bool,
    pub config: Option<ModelConfig>,
    #[serde(rename = "cardData")]
    pub card_data: CardData,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ModelResponse {
    Failed(ErrorResponse),
    Valid(Model),
}

pub fn get_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    if !&CONFIG.huggingface_token.is_empty() {
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&CONFIG.huggingface_token).unwrap(),
        );
    }

    headers
}

pub async fn get_model(username: &String, repo: &String) -> Result<ModelResponse, Error> {
    let request_url = format!("https://huggingface.co/api/models/{username}/{repo}");
    let headers = get_headers();
    let stats = REQ_CLIENT
        .get(&request_url)
        .headers(headers)
        .send()
        .await?
        .json::<ModelResponse>()
        .await?;

    Ok(stats)
}
