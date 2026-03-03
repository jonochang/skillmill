use skillmill_core::PluginRegistry;

pub fn load_registry() -> anyhow::Result<PluginRegistry> {
    let mut registry = PluginRegistry::new();
    registry.register(Box::new(skillmill_math::MathPlugin::new()?));
    Ok(registry)
}
