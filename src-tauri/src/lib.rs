use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let pubsub = elixirkit::PubSub::listen("tcp://127.0.0.1:0")
        .expect("failed to listen on PubSub");
    let pubsub_url = pubsub.url();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(move |app| {
            let app_handle = app.handle().clone();

            // Wait for Elixir to broadcast "ready" before opening a window
            pubsub.subscribe("messages", move |msg| {
                if msg == b"ready" {
                    create_window(&app_handle);
                } else {
                    println!("[rust] {}", String::from_utf8_lossy(msg));
                }
            });

            let app_handle = app.handle().clone();

            tauri::async_runtime::spawn_blocking(move || {
                let rel_dir = app_handle
                    .path()
                    .resource_dir()
                    .unwrap()
                    .join("rel");

                let mut command = elixir_command(&rel_dir, &app_handle, &pubsub_url);
                let status = command.status().expect("failed to start Elixir");
                app_handle.exit(status.code().unwrap_or(1));
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn create_window(app_handle: &tauri::AppHandle) {
    let url = tauri::WebviewUrl::External("http://127.0.0.1:4000".parse().unwrap());
    tauri::WebviewWindowBuilder::new(app_handle, "main", url)
        .title("Qitto")
        .inner_size(1200.0, 800.0)
        .build()
        .unwrap();
}

fn elixir_command(
    rel_dir: &std::path::Path,
    app_handle: &tauri::AppHandle,
    pubsub_url: &str,
) -> std::process::Command {
    if cfg!(debug_assertions) {
        // Development: run mix phx.server from the Phoenix project root
        let mut command = elixirkit::mix("phx.server", &[]);
        command.current_dir("..");
        command.env("ELIXIRKIT_PUBSUB", pubsub_url);
        command
    } else {
        // Production: use bundled Elixir release, store SQLite DB in OS app data dir
        let data_dir = app_handle
            .path()
            .app_data_dir()
            .expect("failed to get app data dir");

        std::fs::create_dir_all(&data_dir).unwrap();
        let db_path = data_dir.join("qitto.db");

        let mut command = elixirkit::release(rel_dir, "qitto");
        command.env("PHX_SERVER", "true");
        command.env("PHX_HOST", "127.0.0.1");
        command.env("PORT", "4000");
        command.env("DATABASE_PATH", db_path.to_str().unwrap());
        command.env(
            "SECRET_KEY_BASE",
            std::env::var("SECRET_KEY_BASE").unwrap_or_else(|_| {
                panic!("SECRET_KEY_BASE env var must be set for production builds")
            }),
        );
        command
    }
}
