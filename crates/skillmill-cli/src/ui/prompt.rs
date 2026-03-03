use dialoguer::{Confirm, Input, Select};

pub fn input(prompt: &str) -> anyhow::Result<String> {
    let value: String = Input::new().with_prompt(prompt).interact_text()?;
    Ok(value)
}

pub fn input_default(prompt: &str, default: &str) -> anyhow::Result<String> {
    let value: String = Input::new()
        .with_prompt(prompt)
        .default(default.to_string())
        .interact_text()?;
    Ok(value)
}

pub fn select(prompt: &str, items: &[String]) -> anyhow::Result<usize> {
    let idx = Select::new().with_prompt(prompt).items(items).default(0).interact()?;
    Ok(idx)
}

pub fn confirm(prompt: &str, default: bool) -> anyhow::Result<bool> {
    let value = Confirm::new().with_prompt(prompt).default(default).interact()?;
    Ok(value)
}
