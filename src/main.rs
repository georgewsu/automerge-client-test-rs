use automerge::{ObjType, AutoCommit, transaction::Transactable};

fn main() {
    println!("starting main");
    let mut doc = AutoCommit::new();
    let contacts = doc.put_object(automerge::ROOT, "contacts", ObjType::List).unwrap();
    let alice = doc.insert_object(&contacts, 0, ObjType::Map).unwrap();
    doc.put(&alice, "name", "Alice").unwrap();
    doc.put(&alice, "email", "alice@example.com").unwrap();
    let bob = doc.insert_object(&contacts, 1, ObjType::Map).unwrap();
    doc.put(&bob, "name", "Bob").unwrap();
    doc.put(&bob, "email", "bob@example.com").unwrap();
    let _data: Vec<u8> = doc.save();
    println!("finished main");
}
