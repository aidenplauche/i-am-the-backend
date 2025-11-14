#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows

mod ui;

use ui::ui_main;

use tokio::runtime::Runtime;

fn main() -> Result<(), i64> {
    println!("Hello, world!");

    let rt = Runtime::new().expect("Failed to create Tokio runtime");
    let _rt_guard = rt.enter();

    std::thread::spawn(move || {
        rt.block_on(async {
            // run some other async tasks here...
            println!("Async task running in Tokio runtime.");
        });
    });

    // egui must run on the main thread
    let result = ui_main();
    if let Err(e) = result {
        eprintln!("Error running eframe: {}", e);
        return Err(1);
    }

    println!("Goodbye, world!");

    Ok(())
}