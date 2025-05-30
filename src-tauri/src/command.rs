use std::collections::HashMap;
use std::sync::Once;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use tauri::{AppHandle, Emitter, Manager};
use tauri_nspanel::ManagerExt;

use crate::fns::{
    setup_menubar_panel_listeners, swizzle_to_menubar_panel, update_menubar_appearance,
};

static INIT: Once = Once::new();

// Global state for clipboard monitoring
static CLIPBOARD_MONITORING: Mutex<bool> = Mutex::new(false);
static PREFERRED_PM: Mutex<String> = Mutex::new(String::new());

#[derive(Clone, serde::Serialize)]
struct TranslationEvent {
    original: String,
    translated: String,
}

#[tauri::command]
pub fn init(app_handle: tauri::AppHandle) {
    INIT.call_once(|| {
        swizzle_to_menubar_panel(&app_handle);
        update_menubar_appearance(&app_handle);
        setup_menubar_panel_listeners(&app_handle);

        // Initialize with npm as default
        *PREFERRED_PM.lock().unwrap() = "npm".to_string();
    });
}

#[tauri::command]
pub fn show_menubar_panel(app_handle: tauri::AppHandle) {
    let panel = app_handle.get_webview_panel("main").unwrap();
    panel.show();
}

#[tauri::command]
pub fn set_preferred_package_manager(package_manager: String) -> Result<(), String> {
    *PREFERRED_PM.lock().unwrap() = package_manager;
    Ok(())
}

#[tauri::command]
pub fn get_preferred_package_manager() -> String {
    PREFERRED_PM.lock().unwrap().clone()
}

#[tauri::command]
pub fn get_monitoring_state() -> bool {
    *CLIPBOARD_MONITORING.lock().unwrap()
}

#[tauri::command]
pub fn toggle_monitoring(app_handle: AppHandle, enabled: bool) -> Result<(), String> {
    let mut monitoring = CLIPBOARD_MONITORING.lock().unwrap();
    *monitoring = enabled;

    if enabled {
        start_clipboard_monitoring(app_handle);
    }

    Ok(())
}

fn start_clipboard_monitoring(app_handle: AppHandle) {
    thread::spawn(move || {
        let mut last_clipboard = String::new();

        loop {
            // Check if monitoring is still enabled
            {
                let monitoring = CLIPBOARD_MONITORING.lock().unwrap();
                if !*monitoring {
                    break;
                }
            }

            // Check clipboard content
            if let Ok(current_clipboard) = get_clipboard_content() {
                if current_clipboard != last_clipboard && !current_clipboard.is_empty() {
                    if let Some(translated) = translate_command(&current_clipboard) {
                        // Update clipboard with translated command
                        if set_clipboard_content(&translated).is_ok() {
                            // Emit event to frontend
                            let _ = app_handle.emit(
                                "command-translated",
                                TranslationEvent {
                                    original: current_clipboard.clone(),
                                    translated: translated.clone(),
                                },
                            );
                        }
                    }
                    last_clipboard = current_clipboard;
                }
            }

            thread::sleep(Duration::from_millis(500));
        }
    });
}

fn translate_command(command: &str) -> Option<String> {
    let command = command.trim();
    let preferred_pm = PREFERRED_PM.lock().unwrap().clone();

    // Check for runners first (npx, bunx, etc.)
    if let Some(translated) = check_and_translate_runners(command, &preferred_pm) {
        return Some(translated);
    }

    // Check for regular package managers
    if let Some(translated) = check_and_translate_package_managers(command, &preferred_pm) {
        return Some(translated);
    }

    None
}

fn check_and_translate_runners(command: &str, preferred_pm: &str) -> Option<String> {
    if command.starts_with("npx ") && preferred_pm != "npm" {
        let args = command.strip_prefix("npx ").unwrap();
        return Some(get_runner_command(preferred_pm, args));
    }

    if command.starts_with("pnpx ") && preferred_pm != "pnpm" {
        let args = command.strip_prefix("pnpx ").unwrap();
        return Some(get_runner_command(preferred_pm, args));
    }

    if command.starts_with("bunx ") && preferred_pm != "bun" {
        let args = command.strip_prefix("bunx ").unwrap();
        return Some(get_runner_command(preferred_pm, args));
    }

    if command.starts_with("yarn dlx ") && preferred_pm != "yarn" {
        let args = command.strip_prefix("yarn dlx ").unwrap();
        return Some(get_runner_command(preferred_pm, args));
    }

    None
}

fn get_runner_command(pm: &str, args: &str) -> String {
    match pm {
        "npm" => format!("npx {}", args),
        "pnpm" => format!("pnpx {}", args),
        "bun" => format!("bunx {}", args),
        "yarn" => format!("yarn dlx {}", args),
        _ => format!("npx {}", args), // fallback to npx
    }
}

fn check_and_translate_package_managers(command: &str, preferred_pm: &str) -> Option<String> {
    let _translations = create_translation_mappings();

    for (pattern, pm_type) in &[
        ("npm ", "npm"),
        ("pnpm ", "pnpm"),
        ("yarn ", "yarn"),
        ("bun ", "bun"),
    ] {
        if command.starts_with(pattern) && pm_type != &preferred_pm {
            return translate_to_preferred_pm(command, pm_type, preferred_pm, &_translations);
        }
    }

    None
}

fn translate_to_preferred_pm(
    command: &str,
    from_pm: &str,
    to_pm: &str,
    _translations: &HashMap<String, HashMap<String, String>>,
) -> Option<String> {
    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.len() < 2 {
        return None;
    }

    let subcommand = parts[1];
    let args = if parts.len() > 2 {
        parts[2..].join(" ")
    } else {
        String::new()
    };

    // Special case translations
    let translated_subcommand = match (from_pm, to_pm, subcommand) {
        // npm -> others
        ("npm", "pnpm" | "yarn" | "bun", "install") => "add",
        ("npm", "pnpm" | "yarn" | "bun", "i") => "add",
        ("npm", "pnpm" | "yarn" | "bun", "uninstall") => "remove",

        // others -> npm
        ("pnpm" | "yarn" | "bun", "npm", "add") => "install",
        ("pnpm" | "yarn" | "bun", "npm", "remove") => "uninstall",

        // yarn specific - no "run" needed for scripts
        ("yarn", _, cmd) if !["add", "remove", "install", "uninstall"].contains(&cmd) => {
            if to_pm == "yarn" {
                cmd // keep as is for yarn
            } else {
                let args_part = if args.is_empty() {
                    String::new()
                } else {
                    format!(" {}", args)
                };
                return Some(format!("{} run {}{}", to_pm, cmd, args_part));
            }
        }

        // Default: keep the same subcommand
        _ => subcommand,
    };

    // Handle global installs
    let translated_command = if args.contains("-g") || args.contains("--global") {
        let clean_args = args
            .replace("-g", "")
            .replace("--global", "")
            .trim()
            .to_string();
        let clean_args_part = if clean_args.is_empty() {
            String::new()
        } else {
            format!(" {}", clean_args)
        };

        match to_pm {
            "pnpm" => format!("pnpm add -g{}", clean_args_part),
            "yarn" => format!("yarn global add{}", clean_args_part),
            "bun" => format!("bun add -g{}", clean_args_part),
            "npm" => format!("npm install -g{}", clean_args_part),
            _ => {
                let args_part = if args.is_empty() {
                    String::new()
                } else {
                    format!(" {}", args)
                };
                format!("{} {}{}", to_pm, translated_subcommand, args_part)
            }
        }
    } else {
        let args_part = if args.is_empty() {
            String::new()
        } else {
            format!(" {}", args)
        };
        format!("{} {}{}", to_pm, translated_subcommand, args_part)
    };

    Some(translated_command)
}

fn create_translation_mappings() -> HashMap<String, HashMap<String, String>> {
    let mut translations = HashMap::new();

    // npm translations
    let mut npm_map = HashMap::new();
    npm_map.insert("install".to_string(), "add".to_string()); // for pnpm/yarn/bun
    npm_map.insert("i".to_string(), "add".to_string());
    npm_map.insert("uninstall".to_string(), "remove".to_string());
    npm_map.insert("run".to_string(), "run".to_string());
    npm_map.insert("start".to_string(), "start".to_string());
    npm_map.insert("test".to_string(), "test".to_string());
    npm_map.insert("build".to_string(), "build".to_string());
    translations.insert("npm".to_string(), npm_map);

    // pnpm translations
    let mut pnpm_map = HashMap::new();
    pnpm_map.insert("add".to_string(), "install".to_string()); // for npm
    pnpm_map.insert("remove".to_string(), "uninstall".to_string());
    pnpm_map.insert("run".to_string(), "run".to_string());
    pnpm_map.insert("start".to_string(), "start".to_string());
    pnpm_map.insert("test".to_string(), "test".to_string());
    pnpm_map.insert("build".to_string(), "build".to_string());
    translations.insert("pnpm".to_string(), pnpm_map);

    // yarn translations
    let mut yarn_map = HashMap::new();
    yarn_map.insert("add".to_string(), "install".to_string()); // for npm
    yarn_map.insert("remove".to_string(), "uninstall".to_string());
    yarn_map.insert("run".to_string(), "run".to_string());
    yarn_map.insert("start".to_string(), "start".to_string());
    yarn_map.insert("test".to_string(), "test".to_string());
    yarn_map.insert("build".to_string(), "build".to_string());
    translations.insert("yarn".to_string(), yarn_map);

    // bun translations
    let mut bun_map = HashMap::new();
    bun_map.insert("add".to_string(), "install".to_string()); // for npm
    bun_map.insert("remove".to_string(), "uninstall".to_string());
    bun_map.insert("run".to_string(), "run".to_string());
    bun_map.insert("start".to_string(), "start".to_string());
    bun_map.insert("test".to_string(), "test".to_string());
    bun_map.insert("build".to_string(), "build".to_string());
    translations.insert("bun".to_string(), bun_map);

    translations
}

#[cfg(target_os = "macos")]
fn get_clipboard_content() -> Result<String, Box<dyn std::error::Error>> {
    use std::process::Command;

    let output = Command::new("pbpaste").output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

#[cfg(target_os = "macos")]
fn set_clipboard_content(content: &str) -> Result<(), Box<dyn std::error::Error>> {
    use std::io::Write;
    use std::process::{Command, Stdio};

    let mut child = Command::new("pbcopy").stdin(Stdio::piped()).spawn()?;

    if let Some(stdin) = child.stdin.as_mut() {
        stdin.write_all(content.as_bytes())?;
    }

    child.wait()?;
    Ok(())
}

#[cfg(not(target_os = "macos"))]
fn get_clipboard_content() -> Result<String, Box<dyn std::error::Error>> {
    // Placeholder for other platforms
    Ok(String::new())
}

#[cfg(not(target_os = "macos"))]
fn set_clipboard_content(_content: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Placeholder for other platforms
    Ok(())
}

#[tauri::command]
pub fn quit_app(app_handle: AppHandle) {
    app_handle.exit(0);
}
