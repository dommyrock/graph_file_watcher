use std::path::Path;
use std::process::Command;

///Executes external processes from current program synchronously
///(meaninig that it blocks current thread)
///
///See: https://stackoverflow.com/questions/62273768/couldnt-convert-the-error-to-stdioerror#:~:text=update%3A%20it%20was%20pointed%20out
///
///Alternative way of detecting OS
///```
/// #[cfg(windows)]
/// pub const NPM: &'static str = "npm.cmd";
/// #[cfg(not(windows))]
/// pub const NPM: &'static str = "npm";
///```
///See: https://stackoverflow.com/questions/57310019/installing-npm-package-with-rust-stdprocesscommand
///```
/// let npm = Path::new("C:\\Program Files\\nodejs");
/// assert!(std::env::set_current_dir(&npm).is_ok());
/// ```
///Example: https://github.com/egoist/dum/blob/main/src/run.rs
pub fn start_node_client() -> Result<(), Box<dyn std::error::Error>> {
    let (sh, sh_flag) = if cfg!(target_os = "windows") {
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
