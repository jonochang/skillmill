use rand::prelude::IndexedRandom;
use rand::Rng;
use rand::RngCore;
use skillmill_core::curriculum::CurriculumGraph;
use skillmill_core::policy::{Band, BandSource};
use skillmill_core::schema::{
    DifficultyAxes, GeneratedItem, RenderedAnswer, RenderedQuestion, SchemaError, SchemaId,
    ValidationResult,
};
use skillmill_core::DisciplinePlugin;
use std::path::{Path, PathBuf};

pub struct MathPlugin {
    curriculum: CurriculumGraph,
    template_dir: PathBuf,
}

impl MathPlugin {
    pub fn new() -> anyhow::Result<Self> {
        let curriculum_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("curricula")
            .join("p1-p3.yaml");
        let curriculum = CurriculumGraph::load_from_file(&curriculum_path)?;
        let template_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../templates/disciplines/math");
        Ok(Self { curriculum, template_dir })
    }

    fn node_id_for_schema(&self, schema_id: &SchemaId) -> Option<String> {
        self.curriculum
            .nodes
            .values()
            .find(|node| node.schemas.iter().any(|s| s == &schema_id.0))
            .map(|node| node.id.0.clone())
    }
}

impl DisciplinePlugin for MathPlugin {
    fn id(&self) -> &'static str {
        "math-singapore"
    }

    fn name(&self) -> &'static str {
        "Singapore MOE Mathematics (P1–P3)"
    }

    fn curriculum(&self) -> &CurriculumGraph {
        &self.curriculum
    }

    fn default_composition(&self) -> Vec<Band> {
        vec![
            Band { source: BandSource::TargetNode, weight: 0.70, item_types: vec!["drill".into()] },
            Band { source: BandSource::Prerequisites, weight: 0.20, item_types: vec!["drill".into()] },
            Band { source: BandSource::NonRoutine, weight: 0.10, item_types: vec!["drill".into()] },
        ]
    }

    fn execute_schema(
        &self,
        schema_id: &SchemaId,
        rng: &mut dyn RngCore,
        _difficulty: &DifficultyAxes,
    ) -> Result<GeneratedItem, SchemaError> {
        let id = schema_id.0.as_str();
        let node_id = self
            .node_id_for_schema(schema_id)
            .ok_or_else(|| SchemaError::NotFound(schema_id.0.clone()))?;
        if id.starts_with("p1-count-to-100") {
            return Ok(generate_count(schema_id, &node_id, rng, 100));
        }

        if id.contains("add-sub-within-10") {
            return Ok(generate_add_sub(schema_id, &node_id, rng, 10));
        }
        if id.contains("add-sub-within-20") {
            return Ok(generate_add_sub(schema_id, &node_id, rng, 20));
        }
        if id.contains("add-sub-within-10000") {
            return Ok(generate_add_sub(schema_id, &node_id, rng, 10_000));
        }
        if id.contains("add-sub-within-100") {
            return Ok(generate_add_sub(schema_id, &node_id, rng, 100));
        }

        if id.contains("multiply-2-3-4-5-10") {
            let tables = [2, 3, 4, 5, 10];
            return Ok(generate_multiply(schema_id, &node_id, rng, &tables));
        }
        if id.contains("divide-2-3-4-5-10") {
            let tables = [2, 3, 4, 5, 10];
            return Ok(generate_divide(schema_id, &node_id, rng, &tables));
        }
        if id.contains("multiply-6-7-8-9") {
            let tables = [6, 7, 8, 9];
            return Ok(generate_multiply(schema_id, &node_id, rng, &tables));
        }
        if id.contains("divide-6-7-8-9") {
            let tables = [6, 7, 8, 9];
            return Ok(generate_divide(schema_id, &node_id, rng, &tables));
        }

        Err(SchemaError::NotFound(schema_id.0.clone()))
    }

    fn validate_answer(&self, item: &GeneratedItem) -> ValidationResult {
        match compute_answer(&item.question.0) {
            Ok(answer) => {
                if answer == item.answer.0 {
                    ValidationResult::ok()
                } else {
                    ValidationResult::fail(format!(
                        "expected {} but got {}",
                        answer, item.answer.0
                    ))
                }
            }
            Err(err) => ValidationResult::fail(err),
        }
    }

    fn template_dir(&self) -> &Path {
        &self.template_dir
    }
}

fn generate_count(
    schema_id: &SchemaId,
    node_id: &str,
    rng: &mut dyn RngCore,
    max: u32,
) -> GeneratedItem {
    let n: u32 = rng.gen_range(1..=max);
    let question = if schema_id.0.ends_with("vertical") {
        format!("Write the number:\n{}", n)
    } else {
        format!("Write the number: {}", n)
    };
    GeneratedItem {
        node_id: node_id.to_string(),
        schema_id: schema_id.clone(),
        question: RenderedQuestion(question),
        answer: RenderedAnswer(n.to_string()),
        working: None,
        visuals: vec![],
    }
}

fn generate_add_sub(
    schema_id: &SchemaId,
    node_id: &str,
    rng: &mut dyn RngCore,
    max: u32,
) -> GeneratedItem {
    let add = rng.gen_bool(0.5);
    let (a, b, op, answer) = if add {
        let a = rng.gen_range(1..=max - 1);
        let b = rng.gen_range(1..=max - a);
        (a, b, '+', a + b)
    } else {
        let a = rng.gen_range(1..=max);
        let b = rng.gen_range(1..=a);
        (a, b, '-', a - b)
    };

    let question = if schema_id.0.ends_with("vertical") {
        format!("{}\n{} {}\n= ___", a, op, b)
    } else {
        format!("{} {} {} = ___", a, op, b)
    };

    GeneratedItem {
        node_id: node_id.to_string(),
        schema_id: schema_id.clone(),
        question: RenderedQuestion(question),
        answer: RenderedAnswer(answer.to_string()),
        working: None,
        visuals: vec![],
    }
}

fn generate_multiply(
    schema_id: &SchemaId,
    node_id: &str,
    rng: &mut dyn RngCore,
    tables: &[u32],
) -> GeneratedItem {
    let a = *tables.choose(rng).unwrap_or(&2);
    let b = rng.gen_range(1..=12);
    let question = if schema_id.0.ends_with("vertical") {
        format!("{}\n* {}\n= ___", a, b)
    } else {
        format!("{} * {} = ___", a, b)
    };

    GeneratedItem {
        node_id: node_id.to_string(),
        schema_id: schema_id.clone(),
        question: RenderedQuestion(question),
        answer: RenderedAnswer((a * b).to_string()),
        working: None,
        visuals: vec![],
    }
}

fn generate_divide(
    schema_id: &SchemaId,
    node_id: &str,
    rng: &mut dyn RngCore,
    tables: &[u32],
) -> GeneratedItem {
    let divisor = *tables.choose(rng).unwrap_or(&2);
    let quotient = rng.gen_range(1..=12);
    let dividend = divisor * quotient;
    let question = if schema_id.0.ends_with("vertical") {
        format!("{}\n/ {}\n= ___", dividend, divisor)
    } else {
        format!("{} / {} = ___", dividend, divisor)
    };

    GeneratedItem {
        node_id: node_id.to_string(),
        schema_id: schema_id.clone(),
        question: RenderedQuestion(question),
        answer: RenderedAnswer(quotient.to_string()),
        working: None,
        visuals: vec![],
    }
}

fn compute_answer(question: &str) -> Result<String, String> {
    let mut nums = Vec::new();
    let mut current = String::new();
    let mut op = None;

    for ch in question.chars() {
        if ch.is_ascii_digit() {
            current.push(ch);
        } else {
            if !current.is_empty() {
                nums.push(current.parse::<u32>().map_err(|e| e.to_string())?);
                current.clear();
            }
            if matches!(ch, '+' | '-' | '*' | '/') {
                op = Some(ch);
            }
        }
    }
    if !current.is_empty() {
        nums.push(current.parse::<u32>().map_err(|e| e.to_string())?);
    }

    if op.is_none() {
        if nums.len() == 1 {
            return Ok(nums[0].to_string());
        }
        return Err("unable to parse operator".to_string());
    }

    let op = op.unwrap();
    if nums.len() < 2 {
        return Err("not enough operands".to_string());
    }
    let a = nums[0];
    let b = nums[1];

    let result = match op {
        '+' => a + b,
        '-' => a.saturating_sub(b),
        '*' => a * b,
        '/' => {
            if b == 0 {
                return Err("division by zero".to_string());
            }
            a / b
        }
        _ => return Err("unknown operator".to_string()),
    };

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_math_plugin_samples() {
        let plugin = MathPlugin::new().expect("plugin init");
        let mut rng = rand::thread_rng();
        for schema in [
            "p1-add-sub-within-10-horizontal",
            "p2-multiply-2-3-4-5-10-vertical",
            "p3-divide-6-7-8-9-horizontal",
        ] {
            let item = plugin
                .execute_schema(&SchemaId(schema.to_string()), &mut rng, &DifficultyAxes::default())
                .expect("execute");
            let validation = plugin.validate_answer(&item);
            assert!(validation.ok, "{schema} validation failed");
        }
    }
}
