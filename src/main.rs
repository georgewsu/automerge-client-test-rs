use memory_stats::memory_stats;
use automerge::{ObjType, AutoCommit, transaction::Transactable};

fn main() {
    println!("starting main");

    if let Some(usage) = memory_stats() {
        println!("Current physical memory usage: {}", usage.physical_mem);
        println!("Current virtual memory usage: {}", usage.virtual_mem);
    }

    let mut doc = AutoCommit::new();
    let contacts = doc.put_object(automerge::ROOT, "contacts", ObjType::List).unwrap();
    let alice = doc.insert_object(&contacts, 0, ObjType::Map).unwrap();
    doc.put(&alice, "name", "Alice").unwrap();
    doc.put(&alice, "email", "alice@example.com").unwrap();
    let bob = doc.insert_object(&contacts, 1, ObjType::Map).unwrap();
    doc.put(&bob, "name", "Bob").unwrap();
    doc.put(&bob, "email", "bob@example.com").unwrap();
    let _data: Vec<u8> = doc.save();

    if let Some(usage) = memory_stats() {
        println!("Current physical memory usage: {}", usage.physical_mem);
        println!("Current virtual memory usage: {}", usage.virtual_mem);
    }

    println!("finished main");
}
