use rand::Rng;
use rand::RngCore;
use rand::prelude::IndexedRandom;
use serde_json::json;
use skillmill_core::DisciplinePlugin;
use skillmill_core::curriculum::CurriculumGraph;
use skillmill_core::policy::{Band, BandSource};
use skillmill_core::schema::{
    DifficultyAxes, GeneratedItem, RenderedAnswer, RenderedQuestion, SchemaError, SchemaId,
    ValidationResult,
};
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
        let template_dir =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../templates/disciplines/math");
        Ok(Self {
            curriculum,
            template_dir,
        })
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
            Band {
                source: BandSource::TargetNode,
                weight: 0.70,
                item_types: vec!["drill".into()],
            },
            Band {
                source: BandSource::Prerequisites,
                weight: 0.20,
                item_types: vec!["drill".into()],
            },
            Band {
                source: BandSource::NonRoutine,
                weight: 0.10,
                item_types: vec!["drill".into()],
            },
        ]
    }

    fn execute_schema(
        &self,
        schema_id: &SchemaId,
        rng: &mut dyn RngCore,
        _difficulty: &DifficultyAxes,
    ) -> Result<GeneratedItem, SchemaError> {
        let node_id = self
            .node_id_for_schema(schema_id)
            .ok_or_else(|| SchemaError::NotFound(schema_id.0.clone()))?;
        match schema_spec(schema_id.0.as_str()) {
            Some(SchemaSpec::Count { max }) => Ok(generate_count(schema_id, &node_id, rng, max)),
            Some(SchemaSpec::AddSub { max }) => Ok(generate_add_sub(schema_id, &node_id, rng, max)),
            Some(SchemaSpec::Multiply { tables }) => {
                Ok(generate_multiply(schema_id, &node_id, rng, tables))
            }
            Some(SchemaSpec::Divide { tables }) => {
                Ok(generate_divide(schema_id, &node_id, rng, tables))
            }
            Some(SchemaSpec::FractionUnit { max_den, component }) => Ok(generate_fraction_unit(
                schema_id, &node_id, rng, max_den, component,
            )),
            Some(SchemaSpec::FractionEquivalent { max_den, component }) => Ok(
                generate_fraction_equivalent(schema_id, &node_id, rng, max_den, component),
            ),
            Some(SchemaSpec::FractionIdentify) => {
                Ok(generate_fraction_identify(schema_id, &node_id, rng))
            }
            Some(SchemaSpec::GeometrySides) => {
                Ok(generate_geometry_sides(schema_id, &node_id, rng))
            }
            Some(SchemaSpec::GeometryVertices) => {
                Ok(generate_geometry_vertices(schema_id, &node_id, rng))
            }
            Some(SchemaSpec::GeometryFaces) => {
                Ok(generate_geometry_faces(schema_id, &node_id, rng))
            }
            None => Err(SchemaError::NotFound(schema_id.0.clone())),
        }
    }

    fn validate_answer(&self, item: &GeneratedItem) -> ValidationResult {
        if let Some(answer) = compute_visual_answer(item) {
            if answer == item.answer.0 {
                return ValidationResult::ok();
            }
            return ValidationResult::fail(format!(
                "expected {} but got {}",
                answer, item.answer.0
            ));
        }

        match compute_answer(&item.question.0) {
            Ok(answer) => {
                if answer == item.answer.0 {
                    ValidationResult::ok()
                } else {
                    ValidationResult::fail(format!("expected {} but got {}", answer, item.answer.0))
                }
            }
            Err(err) => ValidationResult::fail(err),
        }
    }

    fn template_dir(&self) -> &Path {
        &self.template_dir
    }
}

#[derive(Copy, Clone)]
enum SchemaSpec {
    Count {
        max: u32,
    },
    AddSub {
        max: u32,
    },
    Multiply {
        tables: &'static [u32],
    },
    Divide {
        tables: &'static [u32],
    },
    FractionUnit {
        max_den: u32,
        component: FractionComponent,
    },
    FractionEquivalent {
        max_den: u32,
        component: FractionComponent,
    },
    FractionIdentify,
    GeometrySides,
    GeometryVertices,
    GeometryFaces,
}

#[derive(Copy, Clone)]
enum FractionComponent {
    Language,
    Symbols,
    Diagrams,
}

fn schema_spec(id: &str) -> Option<SchemaSpec> {
    // P1
    match id {
        "p1-count-to-100-horizontal" | "p1-count-to-100-vertical" => {
            return Some(SchemaSpec::Count { max: 100 });
        }
        "p1-add-sub-within-10-horizontal" | "p1-add-sub-within-10-vertical" => {
            return Some(SchemaSpec::AddSub { max: 10 });
        }
        "p1-add-sub-within-20-horizontal" | "p1-add-sub-within-20-vertical" => {
            return Some(SchemaSpec::AddSub { max: 20 });
        }
        "p1-geometry-2d-sides-horizontal" | "p1-geometry-2d-sides-vertical" => {
            return Some(SchemaSpec::GeometrySides);
        }
        _ => {}
    }

    // P2
    match id {
        "p2-add-sub-within-100-horizontal" | "p2-add-sub-within-100-vertical" => {
            return Some(SchemaSpec::AddSub { max: 100 });
        }
        "p2-multiply-2-3-4-5-10-horizontal" | "p2-multiply-2-3-4-5-10-vertical" => {
            return Some(SchemaSpec::Multiply {
                tables: &[2, 3, 4, 5, 10],
            });
        }
        "p2-divide-2-3-4-5-10-horizontal" | "p2-divide-2-3-4-5-10-vertical" => {
            return Some(SchemaSpec::Divide {
                tables: &[2, 3, 4, 5, 10],
            });
        }
        "p2-fractions-identify-shaded-horizontal" | "p2-fractions-identify-shaded-vertical" => {
            return Some(SchemaSpec::FractionIdentify);
        }
        "p2-fractions-unit-fractions-language-horizontal"
        | "p2-fractions-unit-fractions-language-vertical" => {
            return Some(SchemaSpec::FractionUnit {
                max_den: 12,
                component: FractionComponent::Language,
            });
        }
        "p2-fractions-unit-fractions-symbols-horizontal"
        | "p2-fractions-unit-fractions-symbols-vertical"
        | "p2-fractions-unit-fractions-horizontal"
        | "p2-fractions-unit-fractions-vertical" => {
            return Some(SchemaSpec::FractionUnit {
                max_den: 12,
                component: FractionComponent::Symbols,
            });
        }
        "p2-fractions-unit-fractions-diagrams-horizontal"
        | "p2-fractions-unit-fractions-diagrams-vertical" => {
            return Some(SchemaSpec::FractionUnit {
                max_den: 12,
                component: FractionComponent::Diagrams,
            });
        }
        "p2-geometry-2d-vertices-horizontal" | "p2-geometry-2d-vertices-vertical" => {
            return Some(SchemaSpec::GeometryVertices);
        }
        _ => {}
    }

    // P3
    match id {
        "p3-add-sub-within-10000-horizontal" | "p3-add-sub-within-10000-vertical" => {
            return Some(SchemaSpec::AddSub { max: 10_000 });
        }
        "p3-multiply-6-7-8-9-horizontal" | "p3-multiply-6-7-8-9-vertical" => {
            return Some(SchemaSpec::Multiply {
                tables: &[6, 7, 8, 9],
            });
        }
        "p3-divide-6-7-8-9-horizontal" | "p3-divide-6-7-8-9-vertical" => {
            return Some(SchemaSpec::Divide {
                tables: &[6, 7, 8, 9],
            });
        }
        "p3-geometry-3d-faces-horizontal" | "p3-geometry-3d-faces-vertical" => {
            return Some(SchemaSpec::GeometryFaces);
        }
        "p3-fractions-equivalent-fractions-language-horizontal"
        | "p3-fractions-equivalent-fractions-language-vertical" => {
            return Some(SchemaSpec::FractionEquivalent {
                max_den: 12,
                component: FractionComponent::Language,
            });
        }
        "p3-fractions-equivalent-fractions-symbols-horizontal"
        | "p3-fractions-equivalent-fractions-symbols-vertical"
        | "p3-fractions-equivalent-fractions-horizontal"
        | "p3-fractions-equivalent-fractions-vertical" => {
            return Some(SchemaSpec::FractionEquivalent {
                max_den: 12,
                component: FractionComponent::Symbols,
            });
        }
        "p3-fractions-equivalent-fractions-diagrams-horizontal"
        | "p3-fractions-equivalent-fractions-diagrams-vertical" => {
            return Some(SchemaSpec::FractionEquivalent {
                max_den: 12,
                component: FractionComponent::Diagrams,
            });
        }
        _ => {}
    }

    None
}

fn generate_count(
    schema_id: &SchemaId,
    node_id: &str,
    rng: &mut dyn RngCore,
    max: u32,
) -> GeneratedItem {
    let n: u32 = rng.random_range(1..=max);
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

fn difficulty_floor(max: u32) -> u32 {
    if max >= 10_000 {
        max / 2
    } else if max >= 100 {
        max / 3
    } else if max >= 20 {
        max / 4
    } else {
        1
    }
}

fn generate_add_sub(
    schema_id: &SchemaId,
    node_id: &str,
    rng: &mut dyn RngCore,
    max: u32,
) -> GeneratedItem {
    let add = rng.random_bool(0.5);
    let min = difficulty_floor(max);
    let (a, b, op, answer) = if add {
        let a = rng.random_range(min..=max - 1);
        let b_max = max.saturating_sub(a).max(1);
        let b = if b_max < min {
            rng.random_range(1..=b_max)
        } else {
            rng.random_range(min..=b_max)
        };
        (a, b, '+', a + b)
    } else {
        let a = rng.random_range(min..=max);
        let b = if a < min {
            rng.random_range(1..=a)
        } else {
            rng.random_range(min..=a)
        };
        (a, b, '-', a - b)
    };

    let _ = schema_id;
    let question = format!("{}\n{} {}\n────", a, op, b);

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
    let b = rng.random_range(1..=12);
    let _ = schema_id;
    let question = format!("{}\n× {}\n────", a, b);

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
    let quotient = rng.random_range(1..=12);
    let dividend = divisor * quotient;
    let _ = schema_id;
    let question = format!("{}\n÷ {}\n────", dividend, divisor);

    GeneratedItem {
        node_id: node_id.to_string(),
        schema_id: schema_id.clone(),
        question: RenderedQuestion(question),
        answer: RenderedAnswer(quotient.to_string()),
        working: None,
        visuals: vec![],
    }
}

fn generate_fraction_unit(
    schema_id: &SchemaId,
    node_id: &str,
    rng: &mut dyn RngCore,
    max_den: u32,
    component: FractionComponent,
) -> GeneratedItem {
    let denominator = rng.random_range(2..=max_den.max(2));
    let question = match component {
        FractionComponent::Language => format!(
            "Language: One part out of {} equal parts is what fraction? ___",
            denominator
        ),
        FractionComponent::Symbols => format!(
            "Symbols: Write the unit fraction with denominator {}: ___",
            denominator
        ),
        FractionComponent::Diagrams => {
            let bar = fraction_bar(1, denominator);
            format!("Diagrams: {}  What fraction is shaded? ___", bar)
        }
    };

    GeneratedItem {
        node_id: node_id.to_string(),
        schema_id: schema_id.clone(),
        question: RenderedQuestion(question),
        answer: RenderedAnswer(format!("1/{}", denominator)),
        working: None,
        visuals: vec![],
    }
}

fn generate_fraction_equivalent(
    schema_id: &SchemaId,
    node_id: &str,
    rng: &mut dyn RngCore,
    max_den: u32,
    component: FractionComponent,
) -> GeneratedItem {
    let denominator = rng.random_range(2..=max_den.max(3));
    let numerator = rng.random_range(1..denominator);
    let factor = rng.random_range(2..=4);
    let expanded_num = numerator * factor;
    let expanded_den = denominator * factor;
    let missing_numerator = rng.random_bool(0.5);

    let (question, answer) = match component {
        FractionComponent::Language => {
            if missing_numerator {
                let q = format!(
                    "Language: {} out of {} equals how many out of {}? ___",
                    numerator, denominator, expanded_den
                );
                (q, expanded_num.to_string())
            } else {
                let q = format!(
                    "Language: {} out of {} equals {} out of how many? ___",
                    numerator, denominator, expanded_num
                );
                (q, expanded_den.to_string())
            }
        }
        FractionComponent::Symbols => {
            if missing_numerator {
                let q = format!(
                    "Symbols: Find the missing number: {}/{} = ___/{}",
                    numerator, denominator, expanded_den
                );
                (q, expanded_num.to_string())
            } else {
                let q = format!(
                    "Symbols: Find the missing number: {}/{} = {}/___",
                    numerator, denominator, expanded_num
                );
                (q, expanded_den.to_string())
            }
        }
        FractionComponent::Diagrams => {
            let left_bar = fraction_bar(numerator, denominator);
            let right_bar = fraction_bar(expanded_num, expanded_den);
            if missing_numerator {
                let q = format!(
                    "Diagrams: {} = {}  Complete: {}/{} = ___/{}",
                    left_bar, right_bar, numerator, denominator, expanded_den
                );
                (q, expanded_num.to_string())
            } else {
                let q = format!(
                    "Diagrams: {} = {}  Complete: {}/{} = {}/___",
                    left_bar, right_bar, numerator, denominator, expanded_num
                );
                (q, expanded_den.to_string())
            }
        }
    };

    GeneratedItem {
        node_id: node_id.to_string(),
        schema_id: schema_id.clone(),
        question: RenderedQuestion(question),
        answer: RenderedAnswer(answer),
        working: None,
        visuals: vec![],
    }
}

fn generate_fraction_identify(
    schema_id: &SchemaId,
    node_id: &str,
    rng: &mut dyn RngCore,
) -> GeneratedItem {
    let style = if rng.random_bool(0.5) { "bar" } else { "stack" };
    let parts = *[2_u32, 3, 4, 5, 6, 8].choose(rng).unwrap_or(&4);
    let shaded = rng.random_range(1..parts);
    let prompt = match rng.random_range(0..6) {
        0 => "What fraction is shaded?",
        1 => "Write the shaded fraction.",
        2 => "Name the shaded part.",
        3 => "What part of the whole is shaded?",
        4 => "Fill in the fraction for the shaded part.",
        _ => "Write the fraction that is shaded.",
    };
    let question = if schema_id.0.ends_with("vertical") {
        format!("{}\n___ / ___", prompt)
    } else {
        format!("{} ___/___", prompt)
    };

    GeneratedItem {
        node_id: node_id.to_string(),
        schema_id: schema_id.clone(),
        question: RenderedQuestion(question),
        answer: RenderedAnswer(format!("{}/{}", shaded, parts)),
        working: None,
        visuals: vec![fraction_visual(style, shaded, parts)],
    }
}

fn generate_geometry_sides(
    schema_id: &SchemaId,
    node_id: &str,
    rng: &mut dyn RngCore,
) -> GeneratedItem {
    let _ = schema_id;
    let shapes = ["triangle", "square", "rectangle", "pentagon", "hexagon"];
    let shape = *shapes.choose(rng).unwrap_or(&"triangle");
    let answer = shape_sides(shape).unwrap_or(3);
    let question = geometry_sides_question(shape, rng);
    GeneratedItem {
        node_id: node_id.to_string(),
        schema_id: schema_id.clone(),
        question: RenderedQuestion(question),
        answer: RenderedAnswer(answer.to_string()),
        working: None,
        visuals: vec![geometry_visual(shape)],
    }
}

fn generate_geometry_vertices(
    schema_id: &SchemaId,
    node_id: &str,
    rng: &mut dyn RngCore,
) -> GeneratedItem {
    let _ = schema_id;
    let shapes = [
        "triangle",
        "square",
        "rectangle",
        "pentagon",
        "hexagon",
        "octagon",
    ];
    let shape = *shapes.choose(rng).unwrap_or(&"triangle");
    let answer = shape_vertices(shape).unwrap_or(3);
    let question = geometry_vertices_question(shape, rng);
    GeneratedItem {
        node_id: node_id.to_string(),
        schema_id: schema_id.clone(),
        question: RenderedQuestion(question),
        answer: RenderedAnswer(answer.to_string()),
        working: None,
        visuals: vec![geometry_visual(shape)],
    }
}

fn generate_geometry_faces(
    schema_id: &SchemaId,
    node_id: &str,
    rng: &mut dyn RngCore,
) -> GeneratedItem {
    let _ = schema_id;
    let solids = [
        ("cube", "cube"),
        ("cuboid", "cuboid"),
        ("triangular prism", "triangular-prism"),
        ("square pyramid", "square-pyramid"),
        ("tetrahedron", "tetrahedron"),
    ];
    let (display_name, lookup_name) = *solids.choose(rng).unwrap_or(&("cube", "cube"));
    let answer = solid_faces(lookup_name).unwrap_or(6);
    let question = geometry_faces_question(display_name, rng);
    GeneratedItem {
        node_id: node_id.to_string(),
        schema_id: schema_id.clone(),
        question: RenderedQuestion(question),
        answer: RenderedAnswer(answer.to_string()),
        working: None,
        visuals: vec![solid_visual(lookup_name)],
    }
}

fn geometry_visual(shape: &str) -> serde_json::Value {
    json!({
        "kind": "shape2d",
        "shape": shape,
    })
}

fn solid_visual(solid: &str) -> serde_json::Value {
    json!({
        "kind": "solid3d",
        "solid": solid,
    })
}

fn fraction_visual(style: &str, shaded: u32, parts: u32) -> serde_json::Value {
    json!({
        "kind": "fraction_bar",
        "style": style,
        "shaded": shaded,
        "parts": parts,
    })
}

fn fraction_bar(shaded: u32, total: u32) -> String {
    let clamped_total = total.clamp(2, 16);
    let clamped_shaded = shaded.min(clamped_total);
    let filled = "■".repeat(clamped_shaded as usize);
    let empty = "□".repeat((clamped_total - clamped_shaded) as usize);
    format!("[{}{}]", filled, empty)
}

fn compute_answer(question: &str) -> Result<String, String> {
    if let Some(geometry_answer) = compute_geometry_answer(question) {
        return Ok(geometry_answer.to_string());
    }
    if let Some(fraction_answer) = compute_fraction_answer(question) {
        return Ok(fraction_answer);
    }

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
            if matches!(ch, '+' | '-' | '*' | '/' | '×' | '÷' | 'x' | 'X') {
                op = Some(match ch {
                    '×' | 'x' | 'X' => '*',
                    '÷' => '/',
                    other => other,
                });
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

fn compute_fraction_answer(question: &str) -> Option<String> {
    if let Some(denominator) = extract_after_marker(question, "denominator ") {
        return Some(format!("1/{}", denominator));
    }

    let compact = question.replace('\n', " ");
    let lower = compact.to_lowercase();

    if compact.contains("___/") || compact.contains("/___") {
        let tokens: Vec<&str> = compact
            .split_whitespace()
            .filter(|token| token.contains('/'))
            .collect();
        if tokens.len() < 2 {
            return None;
        }

        let left = tokens[0];
        let right = tokens[1];
        let (left_num, left_den) = parse_fraction_token(left)?;

        if let Some(stripped) = right.strip_prefix("___/") {
            let right_den = stripped.parse::<u32>().ok()?;
            if left_den == 0 {
                return None;
            }
            return Some((left_num * right_den / left_den).to_string());
        }

        if let Some(stripped) = right.strip_suffix("/___") {
            let right_num = stripped.parse::<u32>().ok()?;
            if left_num == 0 {
                return None;
            }
            return Some((left_den * right_num / left_num).to_string());
        }

        return None;
    }

    if lower.contains("equals how many out of") {
        let nums = extract_numbers(&compact);
        if nums.len() >= 3 {
            let left_num = nums[0];
            let left_den = nums[1];
            let right_den = nums[2];
            if left_den == 0 {
                return None;
            }
            return Some((left_num * right_den / left_den).to_string());
        }
    }

    if lower.contains("out of how many") {
        let nums = extract_numbers(&compact);
        if nums.len() >= 3 {
            let left_num = nums[0];
            let left_den = nums[1];
            let right_num = nums[2];
            if left_num == 0 {
                return None;
            }
            return Some((left_den * right_num / left_num).to_string());
        }
    }

    if lower.contains("what fraction is shaded") {
        if let Some((shaded, total)) = parse_fraction_bar_counts(question) {
            if total > 0 {
                return Some(format!("{}/{}", shaded, total));
            }
        }
    }

    let is_unit_fraction_prompt = lower.contains("unit fraction")
        || lower.contains("fraction symbol")
        || lower.contains("one part out of")
        || lower.contains("one out of")
        || lower.contains("read and write")
        || lower.contains("complete the fraction")
        || lower.contains("equal parts")
        || (compact.contains("1/") && compact.contains("___"));
    if !is_unit_fraction_prompt || compact.contains("___/") || compact.contains("/___") {
        return None;
    }

    let nums = extract_numbers(&compact);
    if nums.len() >= 2 && nums[0] == 1 {
        return Some(format!("1/{}", nums[1]));
    }
    nums.last().map(|denominator| format!("1/{}", denominator))
}

fn compute_visual_answer(item: &GeneratedItem) -> Option<String> {
    let visual = item.visuals.first()?;
    let kind = visual.get("kind")?.as_str()?;
    match kind {
        "fraction_bar" => {
            let shaded = visual.get("shaded")?.as_u64()?;
            let parts = visual.get("parts")?.as_u64()?;
            Some(format!("{}/{}", shaded, parts))
        }
        "shape2d" => {
            let shape = visual.get("shape")?.as_str()?;
            let answer = if item.schema_id.0.contains("sides") {
                shape_sides(shape)?
            } else if item.schema_id.0.contains("vertices") {
                shape_vertices(shape)?
            } else {
                return None;
            };
            Some(answer.to_string())
        }
        "solid3d" => {
            let solid = visual.get("solid")?.as_str()?;
            Some(solid_faces(solid)?.to_string())
        }
        _ => None,
    }
}

fn parse_fraction_bar_counts(input: &str) -> Option<(u32, u32)> {
    let start = input.find('[')?;
    let rest = &input[start + 1..];
    let end_rel = rest.find(']')?;
    let bar = &rest[..end_rel];
    let shaded = bar.chars().filter(|c| *c == '■').count() as u32;
    let empty = bar.chars().filter(|c| *c == '□').count() as u32;
    let total = shaded + empty;
    if total == 0 {
        None
    } else {
        Some((shaded, total))
    }
}

fn extract_after_marker(input: &str, marker: &str) -> Option<u32> {
    let idx = input.find(marker)?;
    let suffix = &input[idx + marker.len()..];
    let digits: String = suffix.chars().take_while(|c| c.is_ascii_digit()).collect();
    if digits.is_empty() {
        None
    } else {
        digits.parse::<u32>().ok()
    }
}

fn extract_numbers(input: &str) -> Vec<u32> {
    let mut nums = Vec::new();
    let mut current = String::new();
    for ch in input.chars() {
        if ch.is_ascii_digit() {
            current.push(ch);
        } else if !current.is_empty() {
            if let Ok(n) = current.parse::<u32>() {
                nums.push(n);
            }
            current.clear();
        }
    }
    if !current.is_empty() {
        if let Ok(n) = current.parse::<u32>() {
            nums.push(n);
        }
    }
    nums
}

fn parse_fraction_token(token: &str) -> Option<(u32, u32)> {
    let parts: Vec<&str> = token.split('/').collect();
    if parts.len() != 2 {
        return None;
    }
    let num = parts[0].parse::<u32>().ok()?;
    let den = parts[1].parse::<u32>().ok()?;
    Some((num, den))
}

fn compute_geometry_answer(question: &str) -> Option<u32> {
    let normalized = question.to_lowercase().replace(['-', '?', ':'], " ");

    if normalized.contains("side") {
        if normalized.contains("triangle") {
            return shape_sides("triangle");
        }
        if normalized.contains("square") {
            return shape_sides("square");
        }
        if normalized.contains("rectangle") {
            return shape_sides("rectangle");
        }
        if normalized.contains("pentagon") {
            return shape_sides("pentagon");
        }
        if normalized.contains("hexagon") {
            return shape_sides("hexagon");
        }
        if normalized.contains("octagon") {
            return shape_sides("octagon");
        }
    }

    if normalized.contains("corner") || normalized.contains("vertice") {
        if normalized.contains("triangle") {
            return shape_vertices("triangle");
        }
        if normalized.contains("square") {
            return shape_vertices("square");
        }
        if normalized.contains("rectangle") {
            return shape_vertices("rectangle");
        }
        if normalized.contains("pentagon") {
            return shape_vertices("pentagon");
        }
        if normalized.contains("hexagon") {
            return shape_vertices("hexagon");
        }
        if normalized.contains("octagon") {
            return shape_vertices("octagon");
        }
    }

    if normalized.contains("face") {
        if normalized.contains("triangular prism") {
            return solid_faces("triangular-prism");
        }
        if normalized.contains("square pyramid") {
            return solid_faces("square-pyramid");
        }
        if normalized.contains("tetrahedron") {
            return solid_faces("tetrahedron");
        }
        if normalized.contains("cuboid") {
            return solid_faces("cuboid");
        }
        if normalized.contains("cube") {
            return solid_faces("cube");
        }
    }

    None
}

fn shape_sides(shape: &str) -> Option<u32> {
    match shape {
        "triangle" => Some(3),
        "square" => Some(4),
        "rectangle" => Some(4),
        "pentagon" => Some(5),
        "hexagon" => Some(6),
        "octagon" => Some(8),
        _ => None,
    }
}

fn shape_vertices(shape: &str) -> Option<u32> {
    shape_sides(shape)
}

fn solid_faces(solid: &str) -> Option<u32> {
    match solid {
        "cube" => Some(6),
        "cuboid" => Some(6),
        "triangular-prism" => Some(5),
        "square-pyramid" => Some(5),
        "tetrahedron" => Some(4),
        _ => None,
    }
}

fn geometry_sides_question(shape: &str, rng: &mut dyn RngCore) -> String {
    let article = article_for(shape);
    match rng.random_range(0..8) {
        0 => format!("How many sides does {article} {shape} have? ___"),
        1 => format!("Count the sides of {article} {shape}: ___"),
        2 => format!(
            "{article_cap} {shape} has ___ sides.",
            article_cap = capitalize(article)
        ),
        3 => format!("Write the number of sides in {article} {shape}: ___"),
        4 => format!("Number of sides in {article} {shape} = ___"),
        5 => format!("How many sides are on {article} {shape}? ___"),
        6 => format!(
            "Fill in the blank: {article_cap} {shape} has ___ sides.",
            article_cap = capitalize(article)
        ),
        _ => format!("Sides of {article} {shape}: ___"),
    }
}

fn geometry_vertices_question(shape: &str, rng: &mut dyn RngCore) -> String {
    let article = article_for(shape);
    match rng.random_range(0..8) {
        0 => format!("How many corners does {article} {shape} have? ___"),
        1 => format!("Count the corners of {article} {shape}: ___"),
        2 => format!(
            "{article_cap} {shape} has ___ corners.",
            article_cap = capitalize(article)
        ),
        3 => format!("Write the number of corners in {article} {shape}: ___"),
        4 => format!("Number of corners in {article} {shape} = ___"),
        5 => format!("How many vertices does {article} {shape} have? ___"),
        6 => format!(
            "Fill in the blank: {article_cap} {shape} has ___ corners.",
            article_cap = capitalize(article)
        ),
        _ => format!("Corners of {article} {shape}: ___"),
    }
}

fn geometry_faces_question(solid: &str, rng: &mut dyn RngCore) -> String {
    let article = article_for(solid);
    match rng.random_range(0..8) {
        0 => format!("How many faces does {article} {solid} have? ___"),
        1 => format!("Count the faces of {article} {solid}: ___"),
        2 => format!(
            "{article_cap} {solid} has ___ faces.",
            article_cap = capitalize(article)
        ),
        3 => format!("Write the number of faces in {article} {solid}: ___"),
        4 => format!("Number of faces in {article} {solid} = ___"),
        5 => format!("How many flat faces are on {article} {solid}? ___"),
        6 => format!(
            "Fill in the blank: {article_cap} {solid} has ___ faces.",
            article_cap = capitalize(article)
        ),
        _ => format!("Faces of {article} {solid}: ___"),
    }
}

fn article_for(noun: &str) -> &'static str {
    match noun.chars().next().unwrap_or('a').to_ascii_lowercase() {
        'a' | 'e' | 'i' | 'o' | 'u' => "an",
        _ => "a",
    }
}

fn capitalize(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        Some(first) => {
            let mut out = String::new();
            out.push(first.to_ascii_uppercase());
            out.push_str(chars.as_str());
            out
        }
        None => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_math_plugin_samples() {
        let plugin = MathPlugin::new().expect("plugin init");
        let mut rng = rand::rng();
        for schema in [
            "p1-add-sub-within-10-horizontal",
            "p2-fractions-identify-shaded-horizontal",
            "p2-multiply-2-3-4-5-10-vertical",
            "p2-fractions-unit-fractions-language-horizontal",
            "p2-fractions-unit-fractions-symbols-vertical",
            "p2-fractions-unit-fractions-diagrams-horizontal",
            "p3-divide-6-7-8-9-horizontal",
            "p3-fractions-equivalent-fractions-language-horizontal",
            "p3-fractions-equivalent-fractions-symbols-vertical",
            "p3-fractions-equivalent-fractions-diagrams-horizontal",
        ] {
            let item = plugin
                .execute_schema(
                    &SchemaId(schema.to_string()),
                    &mut rng,
                    &DifficultyAxes::default(),
                )
                .expect("execute");
            let validation = plugin.validate_answer(&item);
            assert!(validation.ok, "{schema} validation failed");
        }
    }
}
