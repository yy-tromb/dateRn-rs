use chrono::{DateTime, Local};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let target_path = std::env::args().nth(1).expect(
        "\
        \x1b[1;4;31mMaybe no arguments.\x1b[0;1;4m\n\
        Use `dateRn \"the path of target file\"`\x1b[0m\
    ",
    );
    let target_path = std::path::PathBuf::from(target_path);
    let created_time = std::fs::metadata(&target_path).unwrap().created().unwrap();
    let created_date: DateTime<Local> = created_time.into();
    let created_date = created_date.format("%Y-%m-%d").to_string();
    let mut updated_path = target_path.clone();
    updated_path.set_file_name(created_date);
    if let Some(extension) = target_path.extension() {
        updated_path.set_extension(extension);
    }
    std::fs::rename(target_path, updated_path).unwrap();
    Ok(())
}
