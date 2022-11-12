mod library;
mod model;
use crate::library::{command_handler, path_handler};

fn main() {
    path_handler::compose_graph_links();

    //TODO :enable this once Frontend linking is working as expected
    //TODO2: figure out how to refetch JSON data when updated,
    //or have Server pooling of data or some fileWatcher service that resets needed parts (calls window.reload on data change...)

    //Start serving client while processing directory items
    // let _srv = start_node_client();
}
