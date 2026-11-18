use std::process::{Command as ProcessCommand, Stdio};

use clap::Command;
use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;
use strum::{Display, EnumIter, EnumString, IntoEnumIterator};

use crate::shared::cli_error::{CliError, CliResult};

// TODO. Use cargo-generate later for more customization.

/* ========================================================== */
/*                         🦀 MAIN 🦀                         */
/* ========================================================== */

pub fn command_starters() -> Command {
    Command::new("starters").about("Choose and install starter templates")
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[derive(Display, EnumString, EnumIter)]
#[strum(serialize_all = "kebab-case")]
enum StarterTemplate {
    DioxusFullstack,
    Tauri,
    TauriFullstack,
}

pub async fn process_starters() -> CliResult<()> {
    let templates: Vec<StarterTemplate> = StarterTemplate::iter().collect();
    let template_names: Vec<String> = templates.iter().map(|t| t.to_string()).collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a starter template")
        .items(&template_names)
        .default(0)
        .interact()
        .map_err(|_| CliError::validation("Failed to get user selection"))?;

    let selected_template =
        templates.get(selection).ok_or_else(|| CliError::validation("Invalid selection"))?;
    clone_starter_template(selected_template)?;

    Ok(())
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

/// nrzUI repository that ships all starters under `crates/_starters/`.
const STARTERS_REPO: &str = "https://git.zweers.ch/nicorenezweers/nrzUI.git";

/// Repository URL, overridable via `NRZUI_REPO` (e.g. a mirror or local path).
fn starters_repo() -> String {
    std::env::var("NRZUI_REPO").ok().filter(|v| !v.trim().is_empty()).unwrap_or_else(|| STARTERS_REPO.to_string())
}

fn git(args: &[&str], cwd: Option<&std::path::Path>) -> CliResult<()> {
    let mut cmd = ProcessCommand::new("git");
    cmd.args(args).stdout(Stdio::inherit()).stderr(Stdio::inherit());
    if let Some(dir) = cwd {
        cmd.current_dir(dir);
    }
    let output = cmd.output().map_err(|_| CliError::git_clone_failed())?;
    if output.status.success() { Ok(()) } else { Err(CliError::git_clone_failed()) }
}

/// Fetches only `crates/_starters/start-<name>` from the nrzUI repository
/// (shallow + sparse, no full history or other crates) into `./start-<name>`.
fn clone_starter_template(template: &StarterTemplate) -> CliResult<()> {
    let template_name = template.to_string();
    let target = std::path::PathBuf::from(format!("start-{template_name}"));
    if target.exists() {
        return Err(CliError::validation(&format!("Directory `{}` already exists", target.display())));
    }
    println!("Installing {template_name} starter...");

    let tmp = std::env::temp_dir().join(format!("nrzui-starter-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&tmp);
    let tmp_str = tmp.to_string_lossy().to_string();
    let subdir = format!("crates/_starters/start-{template_name}");

    let result = (|| {
        git(&["clone", "--depth", "1", "--filter=blob:none", "--sparse", &starters_repo(), &tmp_str], None)?;
        git(&["sparse-checkout", "set", &subdir], Some(&tmp))?;
        let src = tmp.join(&subdir);
        if !src.is_dir() {
            return Err(CliError::git_clone_failed());
        }
        std::fs::rename(&src, &target)
            .or_else(|_| copy_dir(&src, &target))
            .map_err(|_| CliError::git_clone_failed())
    })();
    let _ = std::fs::remove_dir_all(&tmp);
    result?;

    println!("✅ Successfully installed {template_name} starter into ./{}", target.display());
    Ok(())
}

/// Fallback when `rename` fails (temp dir on another filesystem).
fn copy_dir(src: &std::path::Path, dst: &std::path::Path) -> std::io::Result<()> {
    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let to = dst.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_dir(&entry.path(), &to)?;
        } else {
            std::fs::copy(entry.path(), to)?;
        }
    }
    Ok(())
}
