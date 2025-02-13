use graph_rs_sdk::prelude::*;

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";

pub fn list_thumbnails() {
    let graph = Graph::new(ACCESS_TOKEN);
    let collection = graph.v1().me().drive().list_thumbnails().send().unwrap();
    println!("{:#?}", collection.body());
}
