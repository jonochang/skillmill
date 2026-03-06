use crate::curriculum::CurriculumGraph;
use crate::policy::Band;
use crate::schema::{DifficultyAxes, GeneratedItem, SchemaError, SchemaId, ValidationResult};
use indexmap::IndexMap;
use rand::RngCore;
use std::path::Path;

pub trait DisciplinePlugin: Send + Sync {
    fn id(&self) -> &'static str;
    fn name(&self) -> &'static str;

    fn curriculum(&self) -> &CurriculumGraph;

    fn default_composition(&self) -> Vec<Band>;

    fn execute_schema(
        &self,
        schema_id: &SchemaId,
        rng: &mut dyn RngCore,
        difficulty: &DifficultyAxes,
    ) -> Result<GeneratedItem, SchemaError>;

    fn validate_answer(&self, item: &GeneratedItem) -> ValidationResult;

    fn template_dir(&self) -> &Path;
}

pub struct PluginRegistry {
    plugins: IndexMap<String, Box<dyn DisciplinePlugin>>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            plugins: IndexMap::new(),
        }
    }

    pub fn register(&mut self, plugin: Box<dyn DisciplinePlugin>) {
        self.plugins.insert(plugin.id().to_string(), plugin);
    }

    pub fn get(&self, id: &str) -> Option<&dyn DisciplinePlugin> {
        self.plugins.get(id).map(|p| p.as_ref())
    }

    pub fn ids(&self) -> impl Iterator<Item = &str> {
        self.plugins.keys().map(|s| s.as_str())
    }
}
