use memory_stats::memory_stats;
use random_string::generate;
use automerge::{ObjType, AutoCommit, transaction::Transactable};

fn log_memory_usage() -> () {
    if let Some(usage) = memory_stats() {
        println!("physical memory usage: {}M", usage.physical_mem / 1000000);
    }
}

fn test_automerge() -> () {
    let mut doc = AutoCommit::new();
    let contacts = doc.put_object(automerge::ROOT, "contacts", ObjType::List).unwrap();
    let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    for i in 0..100000 {
        let new_contact = doc.insert_object(&contacts, i, ObjType::Map).unwrap();
        doc.put(&new_contact, "name", generate(50, charset)).unwrap();
        doc.put(&new_contact, "email", generate(50, charset)).unwrap();
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
