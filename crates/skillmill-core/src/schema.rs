use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SchemaId(pub String);

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DifficultyAxes {
    #[serde(default)]
    pub varied: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RenderedQuestion(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RenderedAnswer(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RenderedWorking(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedItem {
    pub node_id: String,
    pub schema_id: SchemaId,
    pub question: RenderedQuestion,
    pub answer: RenderedAnswer,
    pub working: Option<RenderedWorking>,
    #[serde(default)]
    pub visuals: Vec<serde_json::Value>,
}

#[derive(Debug, thiserror::Error)]
pub enum SchemaError {
    #[error("schema not found: {0}")]
    NotFound(String),
    #[error("schema generation failed: {0}")]
    GenerationFailed(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub ok: bool,
    pub message: Option<String>,
}

impl ValidationResult {
    pub fn ok() -> Self {
        Self { ok: true, message: None }
    }

    pub fn fail(msg: impl Into<String>) -> Self {
        Self { ok: false, message: Some(msg.into()) }
    }
}
