use std::time::Instant;

use automerge::{Automerge, ReadDoc};

fn load_am(filename: &str) {
    let bytes = std::fs::read(&filename).unwrap();
    println!("Loading {filename} into automerge ({} bytes)", bytes.len());
    let start_time = Instant::now();

    // This is the slow part.
    // let doc = AutoCommit::load(&bytes).unwrap();
    let doc = Automerge::load(&bytes).unwrap();

    println!("Automerge took {} ms to load", Instant::now().duration_since(start_time).as_millis());

    println!("Getting text content...");
    let (_, text_id) = doc.get(automerge::ROOT, "text").unwrap().unwrap();
    let result = doc.text(text_id).unwrap();
    println!("Text document {} bytes long", result.len());
}

fn main() {
    load_am("C1_fast.am");
    println!();
    println!("Note: The second trace takes about 2 minutes to replay!");
    load_am("C1_slow.am");
}
