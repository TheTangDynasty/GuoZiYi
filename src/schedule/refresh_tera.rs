// use std::path::Path;

// use notify::{RecursiveMode, Result, Watcher};
// use tera::Tera;

// pub fn refresh_tera(mut tera: Tera) -> Result<()> {
//     // Automatically select the best implementation for your platform.
//     let mut watcher = notify::recommended_watcher(|res| match res {
//         Ok(event) => println!("event: {:?}", event),
//         Err(e) => println!("watch error: {:?}", e),
//     })?;

//     // Add a path to be watched. All files and directories at that path and
//     // below will be monitored for changes.
//     watcher.watch(Path::new("."), RecursiveMode::Recursive)?;

//     println!("watching for changes");

//     Ok(())
// }