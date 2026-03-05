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
            Some(SchemaSpec::GeometrySides) => Ok(generate_geometry_sides(schema_id, &node_id, rng)),
            Some(SchemaSpec::GeometryVertices) => {
                Ok(generate_geometry_vertices(schema_id, &node_id, rng))
            }
            Some(SchemaSpec::GeometryFaces) => Ok(generate_geometry_faces(schema_id, &node_id, rng)),
            None => Err(SchemaError::NotFound(schema_id.0.clone())),
        }
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

#[derive(Copy, Clone)]
enum SchemaSpec {
    Count { max: u32 },
    AddSub { max: u32 },
    Multiply { tables: &'static [u32] },
    Divide { tables: &'static [u32] },
    GeometrySides,
    GeometryVertices,
    GeometryFaces,
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
            return Some(SchemaSpec::Multiply { tables: &[2, 3, 4, 5, 10] });
        }
        "p2-divide-2-3-4-5-10-horizontal" | "p2-divide-2-3-4-5-10-vertical" => {
            return Some(SchemaSpec::Divide { tables: &[2, 3, 4, 5, 10] });
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
            return Some(SchemaSpec::Multiply { tables: &[6, 7, 8, 9] });
        }
        "p3-divide-6-7-8-9-horizontal" | "p3-divide-6-7-8-9-vertical" => {
            return Some(SchemaSpec::Divide { tables: &[6, 7, 8, 9] });
        }
        "p3-geometry-3d-faces-horizontal" | "p3-geometry-3d-faces-vertical" => {
            return Some(SchemaSpec::GeometryFaces);
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
    let add = rng.gen_bool(0.5);
    let min = difficulty_floor(max);
    let (a, b, op, answer) = if add {
        let a = rng.gen_range(min..=max - 1);
        let b_max = max.saturating_sub(a).max(1);
        let b = if b_max < min {
            rng.gen_range(1..=b_max)
        } else {
            rng.gen_range(min..=b_max)
        };
        (a, b, '+', a + b)
    } else {
        let a = rng.gen_range(min..=max);
        let b = if a < min {
            rng.gen_range(1..=a)
        } else {
            rng.gen_range(min..=a)
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
    let b = rng.gen_range(1..=12);
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
    let quotient = rng.gen_range(1..=12);
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

fn generate_geometry_sides(schema_id: &SchemaId, node_id: &str, rng: &mut dyn RngCore) -> GeneratedItem {
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
        visuals: vec![],
    }
}

fn generate_geometry_vertices(
    schema_id: &SchemaId,
    node_id: &str,
    rng: &mut dyn RngCore,
) -> GeneratedItem {
    let _ = schema_id;
    let shapes = ["triangle", "square", "rectangle", "pentagon", "hexagon", "octagon"];
    let shape = *shapes.choose(rng).unwrap_or(&"triangle");
    let answer = shape_vertices(shape).unwrap_or(3);
    let question = geometry_vertices_question(shape, rng);
    GeneratedItem {
        node_id: node_id.to_string(),
        schema_id: schema_id.clone(),
        question: RenderedQuestion(question),
        answer: RenderedAnswer(answer.to_string()),
        working: None,
        visuals: vec![],
    }
}

fn generate_geometry_faces(schema_id: &SchemaId, node_id: &str, rng: &mut dyn RngCore) -> GeneratedItem {
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
        if let Some(geometry_answer) = compute_geometry_answer(question) {
            return Ok(geometry_answer.to_string());
        }
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

fn compute_geometry_answer(question: &str) -> Option<u32> {
    let normalized = question
        .to_lowercase()
        .replace('-', " ")
        .replace('?', " ")
        .replace(':', " ");

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
    match rng.gen_range(0..8) {
        0 => format!("How many sides does {article} {shape} have? ___"),
        1 => format!("Count the sides of {article} {shape}: ___"),
        2 => format!("{article_cap} {shape} has ___ sides.", article_cap = capitalize(article)),
        3 => format!("Write the number of sides in {article} {shape}: ___"),
        4 => format!("Number of sides in {article} {shape} = ___"),
        5 => format!("How many sides are on {article} {shape}? ___"),
        6 => format!("Fill in the blank: {article_cap} {shape} has ___ sides.", article_cap = capitalize(article)),
        _ => format!("Sides of {article} {shape}: ___"),
    }
}

fn geometry_vertices_question(shape: &str, rng: &mut dyn RngCore) -> String {
    let article = article_for(shape);
    match rng.gen_range(0..8) {
        0 => format!("How many corners does {article} {shape} have? ___"),
        1 => format!("Count the corners of {article} {shape}: ___"),
        2 => format!("{article_cap} {shape} has ___ corners.", article_cap = capitalize(article)),
        3 => format!("Write the number of corners in {article} {shape}: ___"),
        4 => format!("Number of corners in {article} {shape} = ___"),
        5 => format!("How many vertices does {article} {shape} have? ___"),
        6 => format!("Fill in the blank: {article_cap} {shape} has ___ corners.", article_cap = capitalize(article)),
        _ => format!("Corners of {article} {shape}: ___"),
    }
}

fn geometry_faces_question(solid: &str, rng: &mut dyn RngCore) -> String {
    let article = article_for(solid);
    match rng.gen_range(0..8) {
        0 => format!("How many faces does {article} {solid} have? ___"),
        1 => format!("Count the faces of {article} {solid}: ___"),
        2 => format!("{article_cap} {solid} has ___ faces.", article_cap = capitalize(article)),
        3 => format!("Write the number of faces in {article} {solid}: ___"),
        4 => format!("Number of faces in {article} {solid} = ___"),
        5 => format!("How many flat faces are on {article} {solid}? ___"),
        6 => format!("Fill in the blank: {article_cap} {solid} has ___ faces.", article_cap = capitalize(article)),
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
