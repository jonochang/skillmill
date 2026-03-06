use crate::compose::WorksheetSpec;
use anyhow::Context;
use chrono::Local;
use serde::Serialize;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct RenderPipeline {
    pub template_dir: PathBuf,
}

#[derive(Debug, Clone)]
pub struct RenderResult {
    pub student_pdf: PathBuf,
    pub student_png: Vec<PathBuf>,
    pub answer_key_pdf: Option<PathBuf>,
    pub answer_key_png: Option<Vec<PathBuf>>,
}

#[derive(Serialize)]
struct RenderData<'a> {
    spec: &'a WorksheetSpec,
}

impl RenderPipeline {
    pub fn new(template_dir: impl AsRef<Path>) -> Self {
        Self {
            template_dir: template_dir.as_ref().to_path_buf(),
        }
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
        let student_png = out_dir.join("student.png");
        let resolved_template_dir = resolve_template_dir(&self.template_dir, &spec.template_style);
        let worksheet_template = resolved_template_dir.join("worksheet.typ");
        let answer_template = resolved_template_dir.join("answer-key.typ");
        let answer_pdf = out_dir.join("answer-key.pdf");
        let answer_png = out_dir.join("answer-key.png");

        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .context("failed to build tokio runtime")?;

        let (student_png, answer_key_pdf, answer_key_png) = rt.block_on(async {
            let student = tokio::task::spawn_blocking({
                let worksheet_template = worksheet_template.clone();
                let data_string = data_string.clone();
                let student_pdf = student_pdf.clone();
                let student_png = student_png.clone();
                move || {
                    render_document(
                        &worksheet_template,
                        &data_string,
                        &student_pdf,
                        &student_png,
                    )
                }
            });

            let answer = if include_answer_key {
                Some(tokio::task::spawn_blocking({
                    let answer_template = answer_template.clone();
                    let data_string = data_string.clone();
                    let answer_pdf = answer_pdf.clone();
                    let answer_png = answer_png.clone();
                    move || {
                        render_document(&answer_template, &data_string, &answer_pdf, &answer_png)
                    }
                }))
            } else {
                None
            };

            let student_png = student.await.context("student render task failed")??;

            if let Some(answer) = answer {
                let answer_png = answer.await.context("answer key render task failed")??;
                Ok::<_, anyhow::Error>((student_png, Some(answer_pdf), Some(answer_png)))
            } else {
                Ok::<_, anyhow::Error>((student_png, None, None))
            }
        })?;

        Ok(RenderResult {
            student_pdf,
            student_png,
            answer_key_pdf,
            answer_key_png,
        })
    }
}

fn resolve_template_dir(template_root: &Path, template_style: &str) -> PathBuf {
    let styled_dir = template_root.join(template_style);
    if !template_style.is_empty() && styled_dir.is_dir() {
        styled_dir
    } else {
        template_root.to_path_buf()
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

fn render_document(
    template: &Path,
    data_string: &str,
    pdf_output: &Path,
    png_output: &Path,
) -> anyhow::Result<Vec<PathBuf>> {
    run_typst(template, data_string, pdf_output)?;
    let png_paths = run_typst_png(template, data_string, png_output)?;
    Ok(png_paths)
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

fn run_typst_png(
    template: &Path,
    data_string: &str,
    png_output: &Path,
) -> anyhow::Result<Vec<PathBuf>> {
    let root = PathBuf::from("/");
    let data_arg = data_string.to_string();
    let out_dir = png_output
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."));
    let stem = png_output
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| anyhow::anyhow!("invalid png output path"))?;
    let temp_stem = format!(".skillmill-{stem}");

    clear_existing_png_pages(&out_dir, &temp_stem)?;

    let pattern = out_dir.join(format!("{temp_stem}-{{0p}}.png"));
    let status = std::process::Command::new("typst")
        .arg("compile")
        .arg("--root")
        .arg(&root)
        .arg(template)
        .arg("--input")
        .arg(format!("data={}", data_arg))
        .arg(&pattern)
        .status()
        .with_context(|| "failed to spawn typst for png")?;

    if !status.success() {
        return Err(anyhow::anyhow!("typst png compile failed"));
    }

    collect_png_pages(&out_dir, &temp_stem)
}

fn clear_existing_png_pages(out_dir: &Path, stem: &str) -> anyhow::Result<()> {
    if !out_dir.exists() {
        return Ok(());
    }
    for entry in std::fs::read_dir(out_dir)? {
        let entry = entry?;
        let path = entry.path();
        if is_png_page(&path, stem) {
            std::fs::remove_file(path)?;
        }
    }
    Ok(())
}

fn collect_png_pages(out_dir: &Path, stem: &str) -> anyhow::Result<Vec<PathBuf>> {
    let mut pages = Vec::new();
    for entry in std::fs::read_dir(out_dir)? {
        let entry = entry?;
        let path = entry.path();
        if is_png_page(&path, stem) {
            pages.push(path);
        }
    }
    pages.sort();
    if pages.is_empty() {
        return Err(anyhow::anyhow!("no png pages generated"));
    }
    Ok(pages)
}

fn is_png_page(path: &Path, stem: &str) -> bool {
    let file_name = match path.file_name().and_then(|s| s.to_str()) {
        Some(name) => name,
        None => return false,
    };
    if !file_name.starts_with(&format!("{stem}-")) || !file_name.ends_with(".png") {
        return false;
    }
    let page_part = &file_name[stem.len() + 1..file_name.len() - 4];
    !page_part.is_empty() && page_part.chars().all(|c| c.is_ascii_digit())
}
