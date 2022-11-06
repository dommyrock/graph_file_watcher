use serde::Serialize;
use std::fs::{File, OpenOptions};
use std::io::BufWriter;
use std::path::Path;
use std::process::Command;
use std::{fs, io};

#[derive(Serialize, Debug)]
pub struct Node {
    id: i32,
    name: String,
    kind: String,
}
#[derive(Serialize, Debug)]
pub struct Edge {
    id: i32,
    name: String,
    start: i32,
    end: i32,
    kind: String,
}
pub trait JsonWriter {
    fn write_json(&self, f: BufWriter<File>);
}

impl JsonWriter for String {
    fn write_json(&self, f: BufWriter<File>) {
        serde_json::to_writer(f, self).unwrap();
    }
}
//Extension methods info:
//https://plippe.github.io/blog/2021/06/09/rust-extension-methods.html
//http://xion.io/post/code/rust-extension-traits.html

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let dir_path = &String::from(&args[1]); //1st arg is always path

    let _ = get_file_paths();
    //traverse Directory structure
    let mut nodes: Vec<Node> = vec![];
    let mut edges: Vec<Edge> = vec![];
    let _ = visit_dirs_recurs(Path::new(dir_path), &mut nodes, &mut edges);

    let (node_file, edge_file) = get_file_paths();
    //Write updated Dir structure to files
    let _nodes_payload = serde_json::to_string(&nodes)
        .expect("expected string")
        .write_json(BufWriter::new(node_file));
    let _edges_payload = serde_json::to_string(&edges)
        .expect("expected string")
        .write_json(BufWriter::new(edge_file));

    //TODO :enable this once Frontend linking is working as expected
    //TODO2: figure out how to refetch JSON data when updated, 
    //or have Server pooling of data or some fileWatcher service that resets needed parts

    //Start serving client while processing directory items
    // let _srv = start_node_client();
}

fn visit_dirs_recurs(dir: &Path, nodes: &mut Vec<Node>, edges: &mut Vec<Edge>) -> io::Result<()> {
    let mut layer_id = 0;

    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let path = entry?.path();
            if path.is_dir() {
                nodes.push(Node {
                    id: layer_id,
                    name: path.display().to_string(),
                    kind: String::from("Folder"),
                });

                visit_dirs_recurs(&path, nodes, edges)?; //recurse nested dir
            } else if path.is_file() {
                edges.push(Edge {
                    id: layer_id,
                    name: path.display().to_string(),
                    start: 1,
                    end: 2,
                    kind: String::from("File"),
                });
                println!("{:?} >> is FILE!\n", &path);
            }
            layer_id += 1;
        }
    }
    Ok(())
}

use File as node_file;
use File as edge_file;
pub fn get_file_paths() -> (node_file, edge_file) {
    const BASE_PATH: &str = "D:\\Me\\Git\\graph_file_watcher\\frontend";
    let mut res: Vec<File> = vec![];

    for file_name in vec!["nodes.json", "edges.json"] {
        let pth = format!("{BASE_PATH}\\{file_name}");
        res.push(
            OpenOptions::new()
                .write(true)
                .create(true)
                .open(pth)
                .unwrap(),
        );
    }
    //https://stackoverflow.com/questions/27904864/what-does-cannot-move-out-of-index-of-mean
    return (res.remove(0), res.remove(0));
}

///Executes external processes from current program
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