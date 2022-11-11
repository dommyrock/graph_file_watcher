use serde::Serialize;
use std::fs::{File, OpenOptions};
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{fs, io};

#[derive(Serialize, Debug)]
pub struct Node {
    id: u32,
    name: String,
    kind: String,
}
#[derive(Serialize, Debug)]
pub struct Edge {
    id: u32,
    name: String,
    start: u32,
    end: u32,
    kind: String,
}
#[derive(Clone)]
pub struct DirInfo {
    pth: PathBuf,
    id: u32,
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

    //traverse Directory structure, prepend ROOT Node/Dir
    let mut nodes: Vec<Node> = vec![Node {
        id: 0,
        name: String::from(dir_path),
        kind: String::from("Folder"),
    }];
    let mut edges: Vec<Edge> = vec![];
    let mut current_node: u32 = 1;
    let mut folder_queue: Vec<DirInfo> = Vec::from([DirInfo {
        id: 0,
        pth: PathBuf::from(dir_path),
    }]);

    let _ = gen_directory_links(
        Path::new(dir_path),
        &mut folder_queue,
        &mut nodes,
        &mut edges,
        &mut current_node,
    );

    let (node_file, edge_file) = get_out_file_paths();
    //Write updated Dir structure to files
    let _nodes_payload = serde_json::to_string(&nodes)
        .expect("expected JSON String")
        .write_json(BufWriter::new(node_file));
    let _edges_payload = serde_json::to_string(&edges)
        .expect("expected JSON String")
        .write_json(BufWriter::new(edge_file));

    //TODO :enable this once Frontend linking is working as expected
    //TODO2: figure out how to refetch JSON data when updated,
    //or have Server pooling of data or some fileWatcher service that resets needed parts (calls window.reload on data change...)

    //Start serving client while processing directory items
    // let _srv = start_node_client();
}

///https://stackoverflow.com/questions/37439327/how-to-write-a-function-that-returns-vecpath
///
///https://nick.groenen.me/notes/rust-path-vs-pathbuf
pub fn read_filenames_from_dir<P>(path: P) -> Result<Vec<PathBuf>, io::Error>
where
    P: AsRef<Path>,
{
    fs::read_dir(path)?
        .into_iter()
        .map(|x| x.map(|entry| entry.path()))
        .collect()
}

/* Path > PathBuff Conversions
        let pb: PathBuf = [r"C:\", "windows", "system32.dll"].iter().collect();
        let pth: &Path = pb.as_path();
        let coerced_to_path:&Path = PathBuf::from("/test").as_path();

        https://doc.rust-lang.org/stable/std/path/struct.PathBuf.html#method.as_path
*/

///Links nested Folder -> Files through BFS with queue
///
/// Clone NEEDED! on
/// ```
/// folder_queue.first().cloned()
/// ```
/// I need to update "folder_queue" while having references to it
/// https://stackoverflow.com/questions/47618823/cannot-borrow-as-mutable-because-it-is-also-borrowed-as-immutable
fn gen_directory_links<P>(
    dir: P,
    folder_queue: &mut Vec<DirInfo>,
    nodes: &mut Vec<Node>,
    edges: &mut Vec<Edge>,
    current_node: &mut u32,
) -> io::Result<()>
where
    P: AsRef<Path>,
{
    if dir.as_ref().exists() {
        while !folder_queue.is_empty() {
            if let Some(pb) = folder_queue.first().cloned() {
                if let Ok(d_i_r) = fs::read_dir(&pb.pth) {
                    for entry in d_i_r {
                        let path = &entry?.path();

                        let ext_name = path
                            .file_name()
                            .expect("Failed to get 'filename' from Abs path")
                            .to_str()
                            .expect("Fail OsStr > &str conversion");

                        nodes.push(Node {
                            id: *current_node,
                            name: String::from(ext_name),
                            kind: if path.is_dir() {
                                String::from("Folder")
                            } else {
                                String::from("File")
                            },
                        });

                        edges.push(Edge {
                            id: *current_node,
                            name: String::from(ext_name),
                            start: pb.id,
                            end: *current_node,
                            kind: String::from("File"),
                        });

                        if path.is_dir() {
                            folder_queue.push(DirInfo {
                                pth: path.to_path_buf(),
                                id: *current_node,
                            });
                        }
                        //NEXT NODE
                        *current_node += 1;
                    }
                }
            }
            folder_queue.swap_remove(0);
        }
    }
    Ok(())
}

use File as node_file;
use File as edge_file;
pub fn get_out_file_paths() -> (node_file, edge_file) {
    const BASE_PATH: &str = "D:\\Me\\Git\\graph_file_watcher\\frontend\\graph-ui-ts\\public\\data";

    let mut res: Vec<File> = vec![];

    for file_name in vec!["nodes.json", "edges.json"] {
        let pth = format!("{BASE_PATH}\\{file_name}");
        res.push(
            OpenOptions::new()
                .create(true)
                .truncate(true)
                .write(true)
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
