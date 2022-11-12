use crate::model::model::{DirInfo, Edge, JsonWriter, Node};
use std::fs::{File, OpenOptions};
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use std::{fs, io};

pub fn compose_graph_links() {
    let args = std::env::args().collect::<Vec<String>>();
    let dir_path = &String::from(&args[1]); //1st arg is always path

    let (mut nodes, mut edges, mut current_node, mut folder_queue) = init(dir_path);

    let _ = gen_directory_links(
        Path::new(dir_path),
        &mut folder_queue,
        &mut nodes,
        &mut edges,
        &mut current_node,
    );
    //SERIALIZE DATA TO FILES
    let (node_file, edge_file) = get_output_paths();
    //Write updated Dir structure to files
    let _nodes_payload = serde_json::to_string(&nodes)
        .expect("expected JSON String")
        .write_json(BufWriter::new(node_file));
    let _edges_payload = serde_json::to_string(&edges)
        .expect("expected JSON String")
        .write_json(BufWriter::new(edge_file));
}
fn init(dir_path: &String) -> (Vec<Node>, Vec<Edge>, u32, Vec<DirInfo>) {
    let nodes: Vec<Node> = vec![Node {
        id: 0,
        name: String::from(dir_path),
        kind: String::from("Folder"),
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
pub fn gen_directory_links<P>(
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
                            label: None,
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
pub fn get_output_paths() -> (node_file, edge_file) {
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
