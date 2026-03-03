use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorksheetPolicy {
    pub discipline: String,
    pub target_node: String,
    pub composition: Vec<Band>,
    pub item_count: u32,
    pub include_answer_key: bool,
    pub include_workings: bool,
    #[serde(default)]
    pub custom_sections: Vec<CustomSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Band {
    pub source: BandSource,
    pub weight: f32,
    #[serde(default)]
    pub item_types: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BandSource {
    TargetNode,
    Prerequisites,
    NonRoutine,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomSection {
    pub position: String,
    pub r#type: String,
    pub content: String,
}

impl WorksheetPolicy {
    pub fn load_from_file(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let path = path.as_ref();
        let text = std::fs::read_to_string(path)
            .with_context(|| format!("failed to read policy file: {}", path.display()))?;
        let policy: Self = serde_yaml::from_str(&text)
            .with_context(|| format!("failed to parse policy YAML: {}", path.display()))?;
        Ok(policy)
    }

    pub fn write_to_file(&self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        let path = path.as_ref();
        let yaml = serde_yaml::to_string(self)?;
        std::fs::write(path, yaml)
            .with_context(|| format!("failed to write policy file: {}", path.display()))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn worksheet_policy_round_trip_yaml() {
        let policy = WorksheetPolicy {
            discipline: "math-singapore".to_string(),
            target_node: "p1-add-sub-within-10".to_string(),
            composition: vec![Band {
                source: BandSource::TargetNode,
                weight: 1.0,
                item_types: vec!["drill".to_string()],
            }],
            item_count: 10,
            include_answer_key: true,
            include_workings: false,
            custom_sections: vec![],
        };

        let yaml = serde_yaml::to_string(&policy).expect("serialize");
        let decoded: WorksheetPolicy = serde_yaml::from_str(&yaml).expect("deserialize");
        assert_eq!(decoded.discipline, policy.discipline);
        assert_eq!(decoded.target_node, policy.target_node);
        assert_eq!(decoded.item_count, policy.item_count);
        assert_eq!(decoded.include_answer_key, policy.include_answer_key);
    }
}
