use tauri::{
    image::Image,
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager, PhysicalPosition, Rect, WebviewWindow, WindowEvent,
};
use tauri_plugin_autostart::ManagerExt as AutostartExt;
use tauri_plugin_process::RestartExt;
use tauri_plugin_updater::UpdaterExt;

#[cfg(desktop)]
use tauri_plugin_autostart::MacosLauncher;

fn toggle_window(app: &AppHandle, tray_rect: Option<Rect>) {
    if let Some(window) = app.get_webview_window("main") {
        if window.is_visible().unwrap_or(false) {
            let _ = window.hide();
        } else {
            show_near_tray(&window, tray_rect);
        }
    }
}

fn show_near_tray(window: &WebviewWindow, tray_rect: Option<Rect>) {
    if let Some(rect) = tray_rect {
        if let Ok(size) = window.outer_size() {
            let icon_x = rect.position.x;
            let icon_y = rect.position.y;
            let icon_w = i32::try_from(rect.size.width).unwrap_or(0);
            let icon_h = i32::try_from(rect.size.height).unwrap_or(0);
            let win_w = i32::try_from(size.width).unwrap_or(360);
            let win_h = i32::try_from(size.height).unwrap_or(520);

            let x = icon_x + (icon_w / 2) - (win_w / 2);
            let y = if icon_y > 120 {
                icon_y - win_h - 12
            } else {
                icon_y + icon_h + 12
            };

            let _ = window.set_position(PhysicalPosition::new(x.max(8), y.max(8)));
        }
    }

    let _ = window.unminimize();
    let _ = window.show();
    let _ = window.set_focus();
}

fn hide_main_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.hide();
    }
}

#[tauri::command]
fn get_autostart_enabled(app: AppHandle) -> Result<bool, String> {
    app.autolaunch()
        .is_enabled()
        .map_err(|error| error.to_string())
}

#[tauri::command]
fn set_autostart_enabled(app: AppHandle, enabled: bool) -> Result<(), String> {
    let launcher = app.autolaunch();
    if enabled {
        launcher.enable()
    } else {
        launcher.disable()
    }
    .map_err(|error| error.to_string())
}

#[tauri::command]
fn hide_to_tray(app: AppHandle) {
    hide_main_window(&app);
}

fn spawn_update_check(app: AppHandle) {
    tauri::async_runtime::spawn(async move {
        let Ok(updater) = app.updater() else {
            return;
        };

        if let Ok(Some(update)) = updater.check().await {
            if update.download_and_install(|_, _| {}, || {}).await.is_ok() {
                app.restart();
            }
        }
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default();

    #[cfg(desktop)]
    {
        builder = builder
            .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
                if let Some(window) = app.get_webview_window("main") {
                    show_near_tray(&window, None);
                }
            }))
            .plugin(tauri_plugin_autostart::init(
                MacosLauncher::LaunchAgent,
                Some(vec!["--hidden"]),
            ))
            .plugin(tauri_plugin_process::init())
            .plugin(tauri_plugin_updater::Builder::new().build());
    }

    builder
        .invoke_handler(tauri::generate_handler![
            get_autostart_enabled,
            set_autostart_enabled,
            hide_to_tray
        ])
        .setup(|app| {
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            let show_item = MenuItem::with_id(app, "show", "Zobrazit", true, None::<&str>)?;
            let hide_item = MenuItem::with_id(app, "hide", "Skrýt", true, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", "Ukončit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_item, &hide_item, &quit_item])?;

            let icon = Image::from_bytes(include_bytes!("../icons/icon.png"))?;

            let _tray = TrayIconBuilder::with_id("main")
                .icon(icon)
                .tooltip("Hodinový skokan")
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            show_near_tray(&window, None);
                        }
                    }
                    "hide" => hide_main_window(app),
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        rect,
                        ..
                    } = event
                    {
                        toggle_window(tray.app_handle(), Some(rect));
                    }
                })
                .build(app)?;

            if let Some(window) = app.get_webview_window("main") {
                let hide_on_blur = window.clone();
                window.on_window_event(move |event| match event {
                    WindowEvent::CloseRequested { api, .. } => {
                        api.prevent_close();
                        let _ = hide_on_blur.hide();
                    }
                    WindowEvent::Focused(false) => {
                        let _ = hide_on_blur.hide();
                    }
                    _ => {}
                });

                let _ = window.hide();
            }

            spawn_update_check(app.handle().clone());

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("failed to run Hodinovy skokan");
}
