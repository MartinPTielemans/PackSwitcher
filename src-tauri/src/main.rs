// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod command;
mod fns;
mod tray;

use std::time::Duration;
use tauri::{Emitter, Manager};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            command::init,
            command::show_menubar_panel,
            command::set_preferred_package_manager,
            command::get_preferred_package_manager,
            command::get_monitoring_state,
            command::toggle_monitoring,
            command::quit_app,
            check_for_updates,
            install_update
        ])
        .plugin(tauri_nspanel::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            let app_handle = app.app_handle();

            tray::create(app_handle)?;

            // Check for updates on startup
            let app_handle_clone = app_handle.clone();
            tauri::async_runtime::spawn(async move {
                if let Err(e) = check_for_updates_internal(app_handle_clone).await {
                    eprintln!("Failed to check for updates: {}", e);
                }
            });

            // Start periodic update checker (every hour)
            let app_handle_periodic = app_handle.clone();
            tauri::async_runtime::spawn(async move {
                loop {
                    // Wait 1 hour (3600 seconds) - non-blocking async sleep
                    tokio::time::sleep(Duration::from_secs(300)).await;

                    if let Err(e) = check_for_updates_internal(app_handle_periodic.clone()).await {
                        eprintln!("Periodic update check failed: {}", e);
                    }
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Update checking functions
#[tauri::command]
async fn check_for_updates(app: tauri::AppHandle) -> Result<(), String> {
    check_for_updates_internal(app)
        .await
        .map_err(|e| e.to_string())
}

async fn check_for_updates_internal(
    app: tauri::AppHandle,
) -> Result<(), Box<dyn std::error::Error>> {
    use tauri_plugin_updater::UpdaterExt;

    let update = app.updater_builder().build()?.check().await?;

    if let Some(update) = update {
        // Emit an event to the frontend about the available update
        app.emit("update-available", &update.version)?;
    }

    Ok(())
}

#[tauri::command]
async fn install_update(app: tauri::AppHandle) -> Result<(), String> {
    use tauri_plugin_updater::UpdaterExt;

    let update = app
        .updater_builder()
        .build()
        .map_err(|e| e.to_string())?
        .check()
        .await
        .map_err(|e| e.to_string())?;

    if let Some(update) = update {
        let mut downloaded = 0u64;

        update
            .download_and_install(
                |chunk_length, content_length| {
                    downloaded += chunk_length as u64;

                    // Emit progress event to frontend
                    if let Err(e) = app.emit(
                        "update-progress",
                        serde_json::json!({
                            "downloaded": downloaded,
                            "contentLength": content_length.unwrap_or(0)
                        }),
                    ) {
                        eprintln!("Failed to emit progress event: {}", e);
                    }
                },
                || {
                    // Download finished callback
                    println!("Update downloaded and installed successfully");

                    // Emit finished event to frontend
                    if let Err(e) = app.emit("update-finished", ()) {
                        eprintln!("Failed to emit finished event: {}", e);
                    }
                },
            )
            .await
            .map_err(|e| e.to_string())?;

        // Restart the app after successful installation
        app.restart();
    }

    Ok(())
}
