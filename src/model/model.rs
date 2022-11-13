use serde::Serialize;
use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;

#[derive(Serialize, Debug)]
pub struct Node {
    pub id: u32,
    pub name: String,
    pub kind: String,
    pub size: Option<String>,
}
#[derive(Serialize, Debug)]
pub struct Edge {
    pub id: u32,
    pub name: String,
    pub start: u32,
    pub end: u32,
    pub label: Option<String>,
}
#[derive(Clone)]
pub struct DirInfo {
    pub pth: PathBuf,
    pub id: u32,
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
