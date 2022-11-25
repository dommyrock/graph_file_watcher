mod library;
mod model;
use crate::library::{command_handler, path_handler};

fn main() {
    path_handler::compose_graph_links();
    //Start serving client while processing directory items
    if command_handler::open_url("http://localhost:9000") {
        let _srv = command_handler::start_node_client();
    }
}
