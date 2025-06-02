use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::Once;
use std::thread;

use clipboard_rs::{
    Clipboard, ClipboardContext, ClipboardHandler, ClipboardWatcher, ClipboardWatcherContext,
    WatcherShutdown,
};
use tauri::{AppHandle, Emitter};
use tauri_nspanel::ManagerExt;

use crate::fns::{
    setup_menubar_panel_listeners, swizzle_to_menubar_panel, update_menubar_appearance,
};

static INIT: Once = Once::new();

// Global state for clipboard monitoring
static CLIPBOARD_MONITORING: Mutex<bool> = Mutex::new(false);
static PREFERRED_PM: Mutex<String> = Mutex::new(String::new());
static CLIPBOARD_SHUTDOWN: Mutex<Option<WatcherShutdown>> = Mutex::new(None);

#[derive(Clone, serde::Serialize)]
struct TranslationEvent {
    original: String,
    translated: String,
}

// Clipboard handler for event-driven monitoring
struct ClipboardMonitor {
    app_handle: AppHandle,
    clipboard_ctx: ClipboardContext,
    last_clipboard: String,
}

impl ClipboardMonitor {
    fn new(app_handle: AppHandle) -> Result<Self, String> {
        let clipboard_ctx = ClipboardContext::new()
            .map_err(|e| format!("Failed to create clipboard context: {}", e))?;
        Ok(ClipboardMonitor {
            app_handle,
            clipboard_ctx,
            last_clipboard: String::new(),
        })
    }
}

impl ClipboardHandler for ClipboardMonitor {
    fn on_clipboard_change(&mut self) {
        // Check if monitoring is still enabled
        {
            let monitoring = CLIPBOARD_MONITORING.lock().unwrap();
            if !*monitoring {
                return;
            }
        }

        // Get clipboard content using the new clipboard-rs API
        if let Ok(current_clipboard) = self.clipboard_ctx.get_text() {
            if current_clipboard != self.last_clipboard && !current_clipboard.is_empty() {
                if let Some(translated) = translate_command(&current_clipboard) {
                    // Update clipboard with translated command
                    match self.clipboard_ctx.set_text(translated.clone()) {
                        Ok(_) => {
                            // Emit event to frontend
                            let _ = self.app_handle.emit(
                                "command-translated",
                                TranslationEvent {
                                    original: current_clipboard.clone(),
                                    translated: translated.clone(),
                                },
                            );
                        }
                        Err(e) => {
                            eprintln!(
                                "Failed to update clipboard with translated text: {}. Error: {}",
                                translated.clone(),
                                e
                            );
                        }
                    }
                }
                self.last_clipboard = current_clipboard;
            }
        }
    }
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
        start_clipboard_monitoring(app_handle)?;
    } else {
        stop_clipboard_monitoring();
    }

    Ok(())
}

fn start_clipboard_monitoring(app_handle: AppHandle) -> Result<(), String> {
    // Stop any existing monitoring
    stop_clipboard_monitoring();

    // Create the clipboard monitor
    let monitor = ClipboardMonitor::new(app_handle)?;

    // Create the watcher context
    let mut watcher_ctx = ClipboardWatcherContext::new()
        .map_err(|e| format!("Failed to create clipboard watcher: {}", e))?;

    // Add the monitor as a handler and get the shutdown channel
    let shutdown = watcher_ctx.add_handler(monitor).get_shutdown_channel();

    // Store the shutdown channel
    {
        let mut clipboard_shutdown = CLIPBOARD_SHUTDOWN.lock().unwrap();
        *clipboard_shutdown = Some(shutdown);
    }

    // Start watching in a separate thread
    thread::spawn(move || {
        watcher_ctx.start_watch();
    });

    Ok(())
}

fn stop_clipboard_monitoring() {
    // Stop the clipboard watcher using the shutdown channel
    let mut clipboard_shutdown = CLIPBOARD_SHUTDOWN.lock().unwrap();
    if let Some(shutdown) = clipboard_shutdown.take() {
        shutdown.stop();
    }
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

    if command.starts_with("pnpm dlx ") && preferred_pm != "pnpm" {
        let args = command.strip_prefix("pnpm dlx ").unwrap();
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

#[tauri::command]
pub fn quit_app(app_handle: AppHandle) {
    app_handle.exit(0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pnpm_dlx_conversion() {
        // Test pnpm dlx -> other package managers
        assert_eq!(
            check_and_translate_runners("pnpm dlx create-next-app", "npm"),
            Some("npx create-next-app".to_string())
        );
        assert_eq!(
            check_and_translate_runners("pnpm dlx create-next-app", "yarn"),
            Some("yarn dlx create-next-app".to_string())
        );
        assert_eq!(
            check_and_translate_runners("pnpm dlx create-next-app", "bun"),
            Some("bunx create-next-app".to_string())
        );

        // Test that pnpm dlx stays unchanged when preferred PM is pnpm
        assert_eq!(
            check_and_translate_runners("pnpx create-next-app", "pnpm"),
            None
        );
    }

    #[test]
    fn test_pnpx_conversion() {
        // Test pnpx -> other package managers
        assert_eq!(
            check_and_translate_runners("pnpx create-next-app", "npm"),
            Some("npx create-next-app".to_string())
        );
        assert_eq!(
            check_and_translate_runners("pnpx create-next-app", "yarn"),
            Some("yarn dlx create-next-app".to_string())
        );
        assert_eq!(
            check_and_translate_runners("pnpx create-next-app", "bun"),
            Some("bunx create-next-app".to_string())
        );

        // Test that pnpx stays unchanged when preferred PM is pnpm
        assert_eq!(
            check_and_translate_runners("pnpx create-next-app", "pnpm"),
            None
        );
    }

    #[test]
    fn test_other_runners_to_pnpm() {
        // Test that other runners convert to pnpm dlx (not pnpx)
        assert_eq!(
            check_and_translate_runners("npx create-next-app", "pnpm"),
            Some("pnpx create-next-app".to_string())
        );
        assert_eq!(
            check_and_translate_runners("bunx create-next-app", "pnpm"),
            Some("pnpx create-next-app".to_string())
        );
        assert_eq!(
            check_and_translate_runners("yarn dlx create-next-app", "pnpm"),
            Some("pnpx create-next-app".to_string())
        );
    }

    #[test]
    fn test_get_runner_command() {
        assert_eq!(
            get_runner_command("npm", "create-next-app"),
            "npx create-next-app"
        );
        assert_eq!(
            get_runner_command("pnpm", "create-next-app"),
            "pnpx create-next-app"
        );
        assert_eq!(
            get_runner_command("yarn", "create-next-app"),
            "yarn dlx create-next-app"
        );
        assert_eq!(
            get_runner_command("bun", "create-next-app"),
            "bunx create-next-app"
        );
    }
}
