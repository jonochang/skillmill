use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorksheetCustomisation {
    pub header: WorksheetHeader,
    pub layout: WorksheetLayout,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorksheetHeader {
    #[serde(default)]
    pub school: Option<String>,
    #[serde(default)]
    pub class: Option<String>,
    #[serde(default = "default_date")]
    pub date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorksheetLayout {
    #[serde(default = "default_font_size")]
    pub font_size: u16,
    #[serde(default = "default_working_space")]
    pub working_space: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudentProfile {
    pub name: String,
    pub discipline: String,
    pub current_node: String,
    #[serde(default)]
    pub mastery: std::collections::HashMap<String, String>,
    pub customisations: WorksheetCustomisation,
}

impl StudentProfile {
    pub fn load_from_file(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let path = path.as_ref();
        let text = std::fs::read_to_string(path)
            .with_context(|| format!("failed to read profile file: {}", path.display()))?;
        let profile: Self = serde_yaml::from_str(&text)
            .with_context(|| format!("failed to parse profile YAML: {}", path.display()))?;
        Ok(profile)
    }

    pub fn write_to_file(&self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        let path = path.as_ref();
        let yaml = serde_yaml::to_string(self)?;
        std::fs::write(path, yaml)
            .with_context(|| format!("failed to write profile file: {}", path.display()))?;
        Ok(())
    }
}

fn default_date() -> String {
    "auto".to_string()
}

fn default_font_size() -> u16 {
    12
}

fn default_working_space() -> String {
    "medium".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn student_profile_round_trip_yaml() {
        let profile = StudentProfile {
            name: "Alice".to_string(),
            discipline: "math-singapore".to_string(),
            current_node: "p1-add-sub-within-10".to_string(),
            mastery: std::collections::HashMap::new(),
            customisations: WorksheetCustomisation {
                header: WorksheetHeader {
                    school: Some("Rosyth".to_string()),
                    class: Some("3 Integrity".to_string()),
                    date: "auto".to_string(),
                },
                layout: WorksheetLayout {
                    font_size: 12,
                    working_space: "medium".to_string(),
                },
            },
        };

        let yaml = serde_yaml::to_string(&profile).expect("serialize");
        let decoded: StudentProfile = serde_yaml::from_str(&yaml).expect("deserialize");
        assert_eq!(decoded.name, profile.name);
        assert_eq!(decoded.discipline, profile.discipline);
        assert_eq!(decoded.current_node, profile.current_node);
        assert_eq!(decoded.customisations.layout.font_size, 12);
    }
}
