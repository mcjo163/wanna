use std::fs;
use std::path::PathBuf;

pub fn establish_things_file() -> anyhow::Result<PathBuf> {
    let dirs = directories::ProjectDirs::from("", "", clap::crate_name!())
        .expect("unable to determine save location");
    let data_dir = dirs.data_dir();

    // Ensure application data directory exists.
    if !data_dir.is_dir() {
        fs::create_dir_all(data_dir)?;
    }

    // If the things.json file doesn't exist, initialize it with an empty list.
    let things_path = data_dir.join("things.json");
    if !things_path
        .try_exists()
        .expect("unable to establish save file")
    {
        fs::write(&things_path, "[]")?;
    }

    // Return the path to the data file.
    Ok(things_path)
}
