use crate::compose::WorksheetSpec;
use anyhow::Context;
use serde::Serialize;
use std::path::{Path, PathBuf};
use chrono::Local;

#[derive(Debug, Clone)]
pub struct RenderPipeline {
    pub template_dir: PathBuf,
}

#[derive(Debug, Clone)]
pub struct RenderResult {
    pub student_pdf: PathBuf,
    pub answer_key_pdf: Option<PathBuf>,
}

#[derive(Serialize)]
struct RenderData<'a> {
    spec: &'a WorksheetSpec,
}

impl RenderPipeline {
    pub fn new(template_dir: impl AsRef<Path>) -> Self {
        Self { template_dir: template_dir.as_ref().to_path_buf() }
    }

    pub fn render(
        &self,
        spec: &WorksheetSpec,
        out_dir: impl AsRef<Path>,
        include_answer_key: bool,
    ) -> anyhow::Result<RenderResult> {
        let out_dir = out_dir.as_ref();
        std::fs::create_dir_all(out_dir)
            .with_context(|| format!("failed to create output dir: {}", out_dir.display()))?;

        let data_path = out_dir.join("worksheet.json");
        let spec = resolve_date(spec);
        let json = serde_json::to_string_pretty(&RenderData { spec: &spec })?;
        std::fs::write(&data_path, &json)
            .with_context(|| format!("failed to write data file: {}", data_path.display()))?;
        let data_string = json;

        let student_pdf = out_dir.join("student.pdf");
        let worksheet_template = self.template_dir.join("worksheet.typ");
        let answer_template = self.template_dir.join("answer-key.typ");
        let answer_path = out_dir.join("answer-key.pdf");

        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .context("failed to build tokio runtime")?;

        let answer_key_pdf = rt.block_on(async {
            let student = tokio::task::spawn_blocking({
                let worksheet_template = worksheet_template.clone();
                let data_string = data_string.clone();
                let student_pdf = student_pdf.clone();
                move || run_typst(&worksheet_template, &data_string, &student_pdf)
            });

            let answer = if include_answer_key {
                Some(tokio::task::spawn_blocking({
                    let answer_template = answer_template.clone();
                    let data_string = data_string.clone();
                    let answer_path = answer_path.clone();
                    move || run_typst(&answer_template, &data_string, &answer_path)
                }))
            } else {
                None
            };

            student
                .await
                .context("student render task failed")??;

            if let Some(answer) = answer {
                answer
                    .await
                    .context("answer key render task failed")??;
                Ok::<_, anyhow::Error>(Some(answer_path))
            } else {
                Ok::<_, anyhow::Error>(None)
            }
        })?;

        Ok(RenderResult { student_pdf, answer_key_pdf })
    }
}

fn resolve_date(spec: &WorksheetSpec) -> WorksheetSpec {
    let mut spec = spec.clone();
    if spec.profile.customisations.header.date == "auto" {
        let today = Local::now().format("%Y-%m-%d").to_string();
        spec.profile.customisations.header.date = today;
    }
    spec
}

fn run_typst(template: &Path, data_string: &str, output: &Path) -> anyhow::Result<()> {
    let root = PathBuf::from("/");
    let data_arg = data_string.to_string();
    let status = std::process::Command::new("typst")
        .arg("compile")
        .arg("--root")
        .arg(&root)
        .arg(template)
        .arg("--input")
        .arg(format!("data={}", data_arg))
        .arg(output)
        .status()
        .with_context(|| "failed to spawn typst")?;

    if !status.success() {
        return Err(anyhow::anyhow!("typst compile failed"));
    }
    Ok(())
}
