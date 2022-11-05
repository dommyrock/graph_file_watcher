use std::path::Path;
use std::{fs, io};

fn main() {
    //1st arg is always path
    let args = std::env::args().collect::<Vec<String>>();
    let path = &String::from(&args[1]);

    let _ = visit_dirs_recurs(Path::new(path));
}

fn visit_dirs_recurs(dir: &Path) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let path = entry?.path();
            //recurse nested dirs
            if path.is_dir() {
                visit_dirs_recurs(&path)?;
            } else if path.is_file() {
                println!("{:?} >> is FILE!\n", &path);
            }
        }
    }
    Ok(())
}
