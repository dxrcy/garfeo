use std::fs;
use std::io;
use std::path::Path;

pub fn copy_folder(src: &Path, dest: &Path) -> io::Result<()> {
    if src.is_dir() {
        fs::create_dir_all(dest)?;

        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let entry_path = entry.path();
            let dest_path = dest.join(entry.file_name());

            if entry_path.is_dir() {
                copy_folder(&entry_path, &dest_path)?;
            } else {
                fs::copy(&entry_path, &dest_path)?;
            }
        }
    }
    Ok(())
}
