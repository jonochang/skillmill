use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;
use skillmill_core::plugin::DisciplinePlugin;
use skillmill_core::schema::{DifficultyAxes, SchemaId};

#[test]
fn validate_all_schemas_1000_samples() {
    let plugin = skillmill_math::MathPlugin::new().expect("plugin init");
    let mut rng = StdRng::seed_from_u64(1234);

    for node in plugin.curriculum().nodes.values() {
        for schema in &node.schemas {
            for _ in 0..1000 {
                let item = plugin
                    .execute_schema(
                        &SchemaId(schema.clone()),
                        &mut rng,
                        &DifficultyAxes::default(),
                    )
                    .expect("execute schema");
                let result = plugin.validate_answer(&item);
                assert!(result.ok, "{} failed: {:?}", schema, result.message);
            }
        }
    }
}

#[test]
fn property_no_panics_10000_iterations() {
    let plugin = skillmill_math::MathPlugin::new().expect("plugin init");
    let mut rng = StdRng::seed_from_u64(999);

    let schemas: Vec<String> = plugin
        .curriculum()
        .nodes
        .values()
        .flat_map(|n| n.schemas.clone())
        .collect();

    for _ in 0..10_000 {
        let schema = &schemas[rng.random_range(0..schemas.len())];
        let _ = plugin
            .execute_schema(
                &SchemaId(schema.clone()),
                &mut rng,
                &DifficultyAxes::default(),
            )
            .expect("execute schema");
    }
}
