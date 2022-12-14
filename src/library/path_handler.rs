use crate::library::util;
use crate::model::model::{DirInfo, Edge, JsonWriter, Node};
use std::fs::{File, OpenOptions};
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use std::{fs, io};

pub fn compose_graph_links() {
    let args = std::env::args().collect::<Vec<String>>();
    let dir_path = &String::from(&args[1]); //1st arg is always path

    let (mut nodes, mut edges, mut current_node, mut folder_queue) =
        initialize_linker_data(dir_path);

    let timer = util::console_timer(format!("Linking directory relationships..."));

    let _ = gen_directory_links(
        Path::new(dir_path),
        &mut folder_queue,
        &mut nodes,
        &mut edges,
        &mut current_node,
    );
    util::console_timer_end(timer);

    let node_count = nodes.len() - 1;
    let timer = util::console_timer(format!("Writing [{node_count} Nodes] to Json file..."));

    let (node_file, edge_file) = get_output_paths();
    //Write updated Dir structure to files
    let _nodes_payload = serde_json::to_string(&nodes)
        .expect("expected JSON String")
        .write_json(BufWriter::new(node_file));
    let _edges_payload = serde_json::to_string(&edges)
        .expect("expected JSON String")
        .write_json(BufWriter::new(edge_file));

    util::console_timer_end(timer);
}

fn initialize_linker_data(dir_path: &String) -> (Vec<Node>, Vec<Edge>, u32, Vec<DirInfo>) {
    let nodes: Vec<Node> = vec![Node {
        id: 0,
        name: String::from(dir_path),
        kind: String::from("Folder"),
        size: None,
    }];
    let edges: Vec<Edge> = vec![];
    let current_node: u32 = 1;
    let folder_queue: Vec<DirInfo> = Vec::from([DirInfo {
        id: 0,
        pth: PathBuf::from(dir_path),
    }]);
    return (nodes, edges, current_node, folder_queue);
}

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
                            size: if path.is_file() {
                                let size = util::get_size_metadata(path.to_str().unwrap());
                                match size {
                                    Ok(x) => Some(String::from(format!("{x:?} Kb"))),
                                    _ => Some(String::from("")),
                                }
                            } else {
                                None
                            },
                        });

                        edges.push(Edge {
                            id: *current_node,
                            name: String::from(ext_name),
                            start: pb.id,
                            end: *current_node,
                            label: None,
                        });

                        if path.is_dir() {
                            folder_queue.push(DirInfo {
                                pth: path.to_path_buf(),
                                id: *current_node,
                            });
                        }
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
fn get_output_paths() -> (node_file, edge_file) {
    const LINKER_OUT_PTH: &str = ".\\frontend\\graph-ui-ts\\public\\data";

    let mut res: Vec<File> = vec![];

    for file_name in vec!["nodes.json", "edges.json"] {
        let pth = format!("{LINKER_OUT_PTH}\\{file_name}");
        res.push(
            OpenOptions::new()
                .create(true)
                .truncate(true)
                .write(true)
                .open(pth)
                .unwrap(),
        );
    }
    return (res.swap_remove(0), res.swap_remove(0));
    //https://stackoverflow.com/questions/27904864/what-does-cannot-move-out-of-index-of-mean
}

///https://stackoverflow.com/questions/37439327/how-to-write-a-function-that-returns-vecpath
///
///https://nick.groenen.me/notes/rust-path-vs-pathbuf
fn _read_filenames_from_dir<P>(path: P) -> Result<Vec<PathBuf>, io::Error>
where
    P: AsRef<Path>,
{
    fs::read_dir(path)?
        .into_iter()
        .map(|x| x.map(|entry| entry.path()))
        .collect()
}
#[cfg(not(target_os = "windows"))]
pub fn adjust_canonicalization<P: AsRef<Path>>(p: P) -> String {
    p.as_ref().display().to_string()
}

///Os specific relative path adjustment
/// Example usage
/// ```
/// pub fn main() {
///let path = PathBuf::from(r#"C:\Windows\System32"#)
///     .canonicalize()
///     .unwrap();
///let display_path = adjust_canonicalization(path);
///println!("Output: {}", display_path);
///}
/// ```
#[cfg(target_os = "windows")]
pub fn adjust_canonicalization<P: AsRef<Path>>(p: P) -> String {
    const VERBATIM_PREFIX: &str = r#"\\?\"#;
    let p = p.as_ref().display().to_string();
    if p.starts_with(VERBATIM_PREFIX) {
        p[VERBATIM_PREFIX.len()..].to_string()
    } else {
        p
    }
}

/* Path > PathBuff Conversions
        let pb: PathBuf = [r"C:\", "windows", "system32.dll"].iter().collect();
        let pth: &Path = pb.as_path();
        let coerced_to_path:&Path = PathBuf::from("/test").as_path();

        https://doc.rust-lang.org/stable/std/path/struct.PathBuf.html#method.as_path
*/
