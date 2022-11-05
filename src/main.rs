use std::path::Path;
use std::process::Command;
use std::{env, fs, io};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let path = &String::from(&args[1]); //1st arg is always path

    //traverse Directory structure
    // let _ = visit_dirs_recurs(Path::new(path));

    //Start serving client while processing directory items
    let _srv = spin_up_node_client();
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

///Executes external processes from current program
///
///See: https://stackoverflow.com/questions/62273768/couldnt-convert-the-error-to-stdioerror#:~:text=update%3A%20it%20was%20pointed%20out
pub fn spin_up_node_client() -> Result<(), Box<dyn std::error::Error>> {
    let npm = Path::new("C:\\Program Files\\nodejs");
    assert!(env::set_current_dir(&npm).is_ok());

    let (sh, sh_flag, ) = if cfg!(target_os = "windows") {
        ("cmd", "/C")
    } else {
        ("sh", "-c")
    };

    let npm = Command::new(sh)
        .arg(sh_flag)
        .arg("npm run srv")
        .current_dir(Path::new(
            "D:\\Me\\Git\\graph_file_watcher\\frontend\\graph-ui-ts",
        ))
        .output();

    //When server is started we stay waiting for it so workaround for now is this shotdown cmd
    //1 Get-Process -Id (Get-NetTCPConnection -LocalPort 4173).OwningProcess
    //2 Stop-Process -Id <PID>

    if npm.is_err() {
        let k = npm.unwrap();
        println!("Err --> {k:?}");
    } else {
        println!("Hmmm: {npm:?}")
    }

    Ok(())
}

//Alternative env seek with proc macros
// #[cfg(windows)]
// pub const NPM: &'static str = "npm.cmd";

// #[cfg(not(windows))]
// pub const NPM: &'static str = "npm";

//cmd execution Rust sample
//https://github.com/egoist/dum/blob/main/src/run.rs
//npm.cmd see
//https://stackoverflow.com/questions/57310019/installing-npm-package-with-rust-stdprocesscommand
