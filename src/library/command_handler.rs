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
///
/// Fallback pkill
/// 
///When server is started we stay waiting for it so workaround for now is this shotdown cmd
//1 Get-Process -Id (Get-NetTCPConnection -LocalPort 4173).OwningProcess
//2 Stop-Process -Id <PID>
pub fn start_node_client() -> Result<(), Box<dyn std::error::Error>> {
    let (sh, sh_flag) = get_os_sh_pfx();
    let port = 9000;
    println!("Starting node server @ Port:{port:?}...");
    let npm = Command::new(sh)
        .arg(sh_flag)
        .arg("npm run srv")
        .current_dir(Path::new(
            "D:\\Me\\Git\\graph_file_watcher\\frontend\\graph-ui-ts",
        ))
        .output();

    if npm.is_err() {
        let k = npm.unwrap();
        println!("Err while starting node server on localhost:{port:?} --> {k:?}");
    }

    Ok(())
}

///Get target os sh prefix
fn get_os_sh_pfx<'a>() -> (&'a str, &'a str) {
    if cfg!(target_os = "windows") {
        return ("cmd", "/C");
    }
    return ("sh", "-c");
}
///Open browser or tab (if funning already) at specific url
pub fn open_url(url: &str) -> bool {
    let (sh, sh_flag) = get_os_sh_pfx();

    if let Ok(mut child) = Command::new(sh).arg(sh_flag).arg("start").arg(&url).spawn() {
        if let Ok(status) = child.wait() {
            return status.success();
        }
    }
    false
}
