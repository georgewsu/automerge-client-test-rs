use memory_stats::memory_stats;
use automerge::{ObjType, AutoCommit, transaction::Transactable};

fn log_memory_usage() -> () {
    if let Some(usage) = memory_stats() {
        println!("physical memory usage: {}M", usage.physical_mem / 1000000);
        // println!("virtual memory usage: {}", usage.virtual_mem);
    }
}

fn test_automerge() -> () {
    let mut doc = AutoCommit::new();
    let contacts = doc.put_object(automerge::ROOT, "contacts", ObjType::List).unwrap();
    let alice = doc.insert_object(&contacts, 0, ObjType::Map).unwrap();
    doc.put(&alice, "name", "Alice").unwrap();
    doc.put(&alice, "email", "alice@example.com").unwrap();
    let bob = doc.insert_object(&contacts, 1, ObjType::Map).unwrap();
    doc.put(&bob, "name", "Bob").unwrap();
    doc.put(&bob, "email", "bob@example.com").unwrap();

    for i in 2..100000 {
        // println!("i: {}", i);
        let new_contact = doc.insert_object(&contacts, i, ObjType::Map).unwrap();
        doc.put(&new_contact, "name", "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ").unwrap();
        doc.put(&new_contact, "email", "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ").unwrap();
    }

    let data: Vec<u8> = doc.save();
    println!("doc vec u8: {}", data.len());
}

fn main() {
    println!("starting main");
    log_memory_usage();
    test_automerge();
    log_memory_usage();
    println!("finished main");
}
