use crate::curriculum::NodeId;
use crate::plugin::DisciplinePlugin;
use crate::policy::{BandSource, WorksheetPolicy};
use crate::profile::StudentProfile;
use crate::schema::{DifficultyAxes, GeneratedItem, SchemaError};
use rand::prelude::IndexedRandom;
use rand::RngCore;
use std::collections::HashSet;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WorksheetSpec {
    pub profile: StudentProfile,
    pub policy: WorksheetPolicy,
    pub items: Vec<GeneratedItem>,
    pub sections: Vec<WorksheetSection>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
pub enum WorksheetSection {
    #[serde(rename = "item")]
    Item { number: u32, item: GeneratedItem },
    #[serde(rename = "custom")]
    Custom { position: String, kind: String, content: String },
}

pub struct Composer;

impl Composer {
    pub fn compose(
        plugin: &dyn DisciplinePlugin,
        policy: WorksheetPolicy,
        profile: StudentProfile,
        rng: &mut dyn RngCore,
    ) -> Result<WorksheetSpec, SchemaError> {
        let mut items = Vec::new();
        let mut seen_fingerprints = HashSet::new();
        let band_counts = allocate_band_counts(policy.item_count, &policy);

        for (band, count) in band_counts {
            let (node_ids, difficulty) = resolve_band(plugin, &policy, &band);
            for _ in 0..count {
                let item = sample_unique_item(
                    plugin,
                    rng,
                    &node_ids,
                    &difficulty,
                    &mut seen_fingerprints,
                )?;
                items.push(item);
            }
        }

        let sections = inject_custom_sections(&items, &policy.custom_sections);
        Ok(WorksheetSpec { profile, policy, items, sections })
    }
}

fn sample_unique_item(
    plugin: &dyn DisciplinePlugin,
    rng: &mut dyn RngCore,
    node_ids: &[NodeId],
    difficulty: &DifficultyAxes,
    seen_fingerprints: &mut HashSet<String>,
) -> Result<GeneratedItem, SchemaError> {
    const MAX_UNIQUENESS_RETRIES: usize = 100;

    for _ in 0..MAX_UNIQUENESS_RETRIES {
        let node_id = node_ids.choose(rng).ok_or_else(|| {
            SchemaError::GenerationFailed("no nodes available for band".to_string())
        })?;
        let schema_id = plugin
            .curriculum()
            .node(node_id)
            .and_then(|n| n.schemas.choose(rng))
            .ok_or_else(|| SchemaError::GenerationFailed("no schemas for node".to_string()))?
            .to_string();
        let item = plugin.execute_schema(&crate::schema::SchemaId(schema_id), rng, difficulty)?;
        let fingerprint = item_fingerprint(&item);
        if seen_fingerprints.insert(fingerprint) {
            return Ok(item);
        }
    }

    Err(SchemaError::GenerationFailed(
        "failed to generate a unique item after retries".to_string(),
    ))
}

fn item_fingerprint(item: &GeneratedItem) -> String {
    item
        .question
        .0
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

fn resolve_band(
    plugin: &dyn DisciplinePlugin,
    policy: &WorksheetPolicy,
    band: &crate::policy::Band,
) -> (Vec<NodeId>, DifficultyAxes) {
    let graph = plugin.curriculum();
    let target = NodeId(policy.target_node.clone());
    match band.source {
        BandSource::TargetNode => (vec![target], DifficultyAxes::default()),
        BandSource::Prerequisites => {
            let prereqs = graph.prerequisites(&target);
            if prereqs.is_empty() {
                (vec![target], DifficultyAxes::default())
            } else {
                (prereqs, DifficultyAxes::default())
            }
        }
        BandSource::NonRoutine => (
            vec![target],
            DifficultyAxes { varied: true },
        ),
    }
}

fn allocate_band_counts(item_count: u32, policy: &WorksheetPolicy) -> Vec<(crate::policy::Band, u32)> {
    if policy.composition.is_empty() {
        return Vec::new();
    }

    let mut counts = Vec::new();
    let mut remaining = item_count as i32;
    for (idx, band) in policy.composition.iter().enumerate() {
        let is_last = idx == policy.composition.len() - 1;
        let mut count = ((item_count as f32) * band.weight).round() as i32;
        if is_last {
            count = remaining;
        }
        if count < 0 {
            count = 0;
        }
        remaining -= count;
        counts.push((band.clone(), count as u32));
    }

    counts
}

fn inject_custom_sections(
    items: &[GeneratedItem],
    custom_sections: &[crate::policy::CustomSection],
) -> Vec<WorksheetSection> {
    let mut sections = Vec::new();
    let mut custom_map: Vec<(u32, &crate::policy::CustomSection, bool)> = Vec::new();

    for section in custom_sections {
        if let Some((pos, before)) = parse_position(&section.position) {
            custom_map.push((pos, section, before));
        }
    }

    for (idx, item) in items.iter().enumerate() {
        let number = (idx + 1) as u32;
        for (_pos, section, _before) in custom_map.iter().filter(|(pos, _, before)| *pos == number && *before) {
            sections.push(WorksheetSection::Custom {
                position: section.position.clone(),
                kind: section.r#type.clone(),
                content: section.content.clone(),
            });
        }
        sections.push(WorksheetSection::Item { number, item: item.clone() });
        for (_pos, section, _before) in custom_map.iter().filter(|(pos, _, before)| *pos == number && !*before) {
            sections.push(WorksheetSection::Custom {
                position: section.position.clone(),
                kind: section.r#type.clone(),
                content: section.content.clone(),
            });
        }
    }

    sections
}

fn parse_position(position: &str) -> Option<(u32, bool)> {
    let parts: Vec<&str> = position.split_whitespace().collect();
    if parts.len() != 2 {
        return None;
    }
    let before = match parts[0] {
        "before_item" => true,
        "after_item" => false,
        _ => return None,
    };
    let index = parts[1].parse::<u32>().ok()?;
    Some((index, before))
}
