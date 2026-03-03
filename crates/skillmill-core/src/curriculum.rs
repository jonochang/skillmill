use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct NodeId(pub String);

impl NodeId {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Level(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: NodeId,
    pub label: String,
    pub level: Level,
    #[serde(default)]
    pub prerequisites: Vec<NodeId>,
    #[serde(default)]
    pub schemas: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurriculumGraph {
    pub nodes: HashMap<NodeId, Node>,
}

impl CurriculumGraph {
    pub fn load_from_file(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let path = path.as_ref();
        let text = std::fs::read_to_string(path)
            .with_context(|| format!("failed to read curriculum file: {}", path.display()))?;
        let graph: Self = serde_yaml::from_str(&text)
            .with_context(|| format!("failed to parse curriculum YAML: {}", path.display()))?;
        Ok(graph)
    }

    pub fn prerequisites(&self, id: &NodeId) -> Vec<NodeId> {
        self.nodes
            .get(id)
            .map(|n| n.prerequisites.clone())
            .unwrap_or_default()
    }

    pub fn node(&self, id: &NodeId) -> Option<&Node> {
        self.nodes.get(id)
    }

    pub fn node_ids(&self) -> impl Iterator<Item = &NodeId> {
        self.nodes.keys()
    }

    pub fn levels(&self) -> Vec<Level> {
        let mut set = HashSet::new();
        for node in self.nodes.values() {
            set.insert(node.level.clone());
        }
        let mut levels: Vec<_> = set.into_iter().collect();
        levels.sort_by(|a, b| a.0.cmp(&b.0));
        levels
    }
}
