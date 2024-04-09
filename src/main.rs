use std::collections::HashMap;
// use std::{thread, time};
use memory_stats::memory_stats;
use random_string::generate;
// use automerge::{ObjType, AutoCommit, transaction::Transactable};
use automerge_repo::{DocumentId, Repo, RepoHandle, Storage, StorageError};
use futures::future::{BoxFuture};
use autosurgeon::{Hydrate, reconcile, Reconcile};
use std::cell::RefCell;

#[derive(Default, Debug, Clone, Reconcile, Hydrate, PartialEq)]
struct DocumentData {
    pub string: String,
    pub strings: Vec<String>,
}

struct HashMapStorage {
    docs: HashMap<DocumentId, Vec<u8>>
}

impl HashMapStorage {
    fn new() -> Self {
        HashMapStorage {
            docs: HashMap::new()
        }
    }
    fn append_doc(&mut self, id: DocumentId, changes: Vec<u8>) -> () {
        self.docs.insert(id, changes);
    }
    fn list_all(&self) -> () {
        for (key, value) in &self.docs {
            println!("{}: {}", key, value.len());
        }
    }
}

struct SimpleStorage {
    hash_map_storage: RefCell<HashMapStorage>
}

impl SimpleStorage {
    fn new() -> Self {
        SimpleStorage {
            hash_map_storage: RefCell::new(HashMapStorage::new())
        }
    }

    fn append_doc(&self, id: DocumentId, changes: Vec<u8>) -> () {
        self.hash_map_storage.borrow_mut().append_doc(id, changes);
    }

    fn list_all_from_storage(&self) -> () {
        self.hash_map_storage.borrow().list_all();
    }
}

impl Storage for SimpleStorage {
    fn get(&self, _id: DocumentId) -> BoxFuture<'static, Result<Option<Vec<u8>>, StorageError>> {
        println!("get: {}", _id);
        Box::pin(futures::future::ready(Ok(None)))
    }

    fn list_all(&self) -> BoxFuture<'static, Result<Vec<DocumentId>, StorageError>> {
        println!("list_all");
        self.list_all_from_storage();
        Box::pin(futures::future::ready(Ok(vec![])))
    }

    fn append(
        &self,
        _id: DocumentId,
        _changes: Vec<u8>,
    ) -> BoxFuture<'static, Result<(), StorageError>> {
        println!("append: {}", _id);
        self.append_doc(_id.clone(), _changes.clone());
        Box::pin(futures::future::ready(Ok(())))
    }

    fn compact(
        &self,
        _id: DocumentId,
        _full_doc: Vec<u8>,
    ) -> BoxFuture<'static, Result<(), StorageError>> {
        println!("compact: {}", _id);
        Box::pin(futures::future::ready(Ok(())))
    }
}

fn log_memory_usage() -> () {
    if let Some(usage) = memory_stats() {
        println!("physical memory usage: {}M", usage.physical_mem / 1_000_000);
        println!("virtual memory usage: {}M", usage.virtual_mem / 1_000_000);
    }
}

/*
fn test_automerge() -> AutoCommit {
    let mut doc = AutoCommit::new();
    let contacts = doc.put_object(automerge::ROOT, "contacts", ObjType::List).unwrap();
    let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    for i in 0..100000 {
        let new_contact = doc.insert_object(&contacts, i, ObjType::Map).unwrap();
        doc.put(&new_contact, "name", generate(50, charset)).unwrap();
        doc.put(&new_contact, "email", generate(50, charset)).unwrap();
    }
    // let data: Vec<u8> = doc.save();
    // println!("doc vec u8: {}", data.len());
    doc
}
*/

fn generate_string_vec(vec_size: u32, string_size: u32) -> Vec<String> {
    // TODO: optimize
    let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut vec: Vec<String> = Vec::with_capacity(vec_size as usize);
    for _ in 0..vec_size {
        vec.push(generate(string_size as usize, charset));
    }
    return vec;
}

fn create_doc(repo_handle: &RepoHandle) -> () {
    let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

    let doc_data: DocumentData = DocumentData {
        string: generate(1000, charset),
        strings: generate_string_vec(10_000, 1000),
        ..Default::default()
    };

    let doc_handle = repo_handle.new_document();
    doc_handle.with_doc_mut(|doc| {
        let mut tx = doc.transaction();
        reconcile(&mut tx, &doc_data).unwrap();
        tx.commit();
    });
    let _doc_id = doc_handle.document_id();
    // println!("created doc: {}", doc_id);
}

fn generate_data(vec_size: u32, string_size: u32) -> DocumentData {
    let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    DocumentData {
        string: generate(string_size as usize, charset),
        strings: generate_string_vec(vec_size, string_size),
        ..Default::default()
    }
}

fn main() {
    println!("starting main");
    log_memory_usage();

    // test_automerge();

    let storage = SimpleStorage::new();
    let repo = Repo::new(None, Box::new(storage));
    let repo_handle = repo.run();

    for _ in 0..10 {
        // generate_data(10_000, 1000);
        create_doc(&repo_handle);
    }

    log_memory_usage();

    let list_result_future = repo_handle.list_all();
    let _list_result = futures::executor::block_on(list_result_future);

    log_memory_usage();

    // thread::sleep(time::Duration::from_millis(5000));
    // log_memory_usage();

    println!("finished main");
}
