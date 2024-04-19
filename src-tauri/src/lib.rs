#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            #[cfg(desktop)]
            desktop::scan_area
        ])
        .setup(|app| {
            #[cfg(desktop)]
            desktop::setup(app)?;

            #[cfg(mobile)]
            mobile::setup(app)?;

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_app, _event| {
            #[cfg(desktop)]
            if let tauri::RunEvent::ExitRequested { api, .. } = _event {
                api.prevent_exit();
            }
        });
}

#[cfg(mobile)]
mod mobile {
    pub fn setup(app: &tauri::App) -> Result<(), Box<dyn std::error::Error>> {
        app.handle().plugin(tauri_plugin_barcode_scanner::init())?;
        app.handle().plugin(tauri_plugin_dialog::init())?;

        tauri::WebviewWindow::builder(app, "main", Default::default()).build()?;

        Ok(())
    }
}

#[cfg(desktop)]
mod desktop {
    use rand::distributions::{Alphanumeric, DistString};
    use tauri::Manager;
    use tauri_plugin_clipboard_manager::ClipboardExt;

    use screenshots::image;

    pub fn setup(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(target_os = "macos")]
        app.set_activation_policy(tauri::ActivationPolicy::Accessory);

        app.handle()
            .plugin(tauri_plugin_clipboard_manager::init())?;

        tauri::tray::TrayIconBuilder::new()
            .icon(app.default_window_icon().unwrap().to_owned())
            .menu(&tauri::menu::Menu::with_items(
                app,
                &[
                    &tauri::menu::MenuItem::with_id(app, "scan", "Scan", true, None::<&str>)?,
                    &tauri::menu::MenuItem::with_id(app, "crop", "Crop", true, None::<&str>)?,
                    &tauri::menu::MenuItem::with_id(
                        app,
                        "clipboard",
                        "Load From Clipboard",
                        true,
                        None::<&str>,
                    )?,
                ],
            )?)
            .on_menu_event(|app, event| match event.id.as_ref() {
                "scan" => {
                    let screens = screenshots::Screen::all().unwrap();
                    let screen = screens.first().unwrap();
                    match screen.capture() {
                        Ok(img) => {
                            let decoded = find_qrcode(&img);
                            process_decoded(app, &decoded);
                        }
                        Err(e) => {
                            eprintln!("failed to capture img: {e}");
                        }
                    }
                }
                "clipboard" => {
                    if let Ok(image) = app.clipboard().read_image() {
                        if let Some(img) = image::load_from_memory(image.rgba()).unwrap().as_rgba8()
                        {
                            let decoded = find_qrcode(img);
                            process_decoded(app, &decoded);
                        } else {
                            eprintln!("failed to parse clipboard image");
                        }
                    } else {
                        eprintln!("clipboard does not contain an image");
                    }
                }
                "crop" => {
                    tauri::WebviewWindow::builder(
                        app,
                        Alphanumeric.sample_string(&mut rand::thread_rng(), 16),
                        Default::default(),
                    )
                    .menu(tauri::menu::Menu::new(app).unwrap())
                    //.inner_size(screen_size.width, screen_size.height)
                    .fullscreen(true)
                    .transparent(true)
                    .always_on_top(true)
                    .build()
                    .unwrap();
                }
                _ => unreachable!(),
            })
            .build(app)?;

        Ok(())
    }

    #[tauri::command]
    pub async fn scan_area(
        window: tauri::WebviewWindow,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
    ) -> Vec<String> {
        let screens = screenshots::Screen::all().unwrap();
        let screen = screens.first().unwrap();

        let capture = screen.capture_area(x, y, width, height + 48);

        match capture {
            Ok(image) => {
                let decoded = find_qrcode(&image);

                process_decoded(window.app_handle(), &decoded);

                if !decoded.is_empty() {
                    std::thread::spawn(move || {
                        std::thread::sleep(std::time::Duration::from_millis(200));
                        window.close().unwrap();
                    });
                }

                decoded
            }
            Err(e) => {
                eprintln!("failed to capture area: {e}");
                Vec::with_capacity(0)
            }
        }
    }

    fn find_qrcode(image: &image::RgbaImage) -> Vec<String> {
        let decoder = bardecoder::default_decoder();
        decoder.decode(image).into_iter().flatten().collect()
    }

    fn process_decoded(_app: &tauri::AppHandle, decoded: &[String]) {
        let write_result =
            _app.clipboard()
                .write_text(tauri_plugin_clipboard_manager::ClipKind::PlainText {
                    label: None,
                    text: decoded.iter().cloned().collect(),
                });
        match write_result {
            Ok(_) => (),
            Err(e) => eprintln!("failed write to clipboard: {e}"),
        }
    }
}
