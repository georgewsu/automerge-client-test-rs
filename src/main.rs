use std::collections::HashMap;
use memory_stats::memory_stats;
use random_string::generate;
// use automerge::{ObjType, AutoCommit, transaction::Transactable};
use automerge_repo::{DocumentId, Repo, RepoHandle, Storage, StorageError};
use futures::future::{BoxFuture};
use autosurgeon::{Hydrate, reconcile, Reconcile};

#[derive(Debug, Clone, Reconcile, Hydrate, PartialEq)]
struct Customer {
    pub number: u32,
    pub views_of_others: HashMap<String, u32>,
}

#[derive(Default, Debug, Clone, Reconcile, Hydrate, PartialEq)]
struct Bakery {
    pub customers: HashMap<String, Customer>,
    pub output: u32,
    pub closing: bool,
    pub string: String,
}

struct NoStorage;

impl Storage for NoStorage {
    fn get(&self, _id: DocumentId) -> BoxFuture<'static, Result<Option<Vec<u8>>, StorageError>> {
        Box::pin(futures::future::ready(Ok(None)))
    }

    fn list_all(&self) -> BoxFuture<'static, Result<Vec<DocumentId>, StorageError>> {
        Box::pin(futures::future::ready(Ok(vec![])))
    }

    fn append(
        &self,
        _id: DocumentId,
        _changes: Vec<u8>,
    ) -> BoxFuture<'static, Result<(), StorageError>> {
        Box::pin(futures::future::ready(Ok(())))
    }

    fn compact(
        &self,
        _id: DocumentId,
        _full_doc: Vec<u8>,
    ) -> BoxFuture<'static, Result<(), StorageError>> {
        Box::pin(futures::future::ready(Ok(())))
    }
}

fn log_memory_usage() -> () {
    if let Some(usage) = memory_stats() {
        println!("physical memory usage: {}M", usage.physical_mem / 1000000);
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

fn create_doc(repo_handle: &RepoHandle) -> () {
    let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

    let bakery: Bakery = Bakery {
        output: 0,
        closing: false,
        string: generate(1000000, charset),
        ..Default::default()
    };

    let doc_handle = repo_handle.new_document();
    doc_handle.with_doc_mut(|doc| {
        let mut tx = doc.transaction();
        reconcile(&mut tx, &bakery).unwrap();
        tx.commit();
    });
}

fn main() {
    println!("starting main");
    log_memory_usage();

    // test_automerge();

    let repo = Repo::new(None, Box::new(NoStorage));
    let repo_handle = repo.run();

    for _ in 0..100 {
        create_doc(&repo_handle);
    }

    // let list_result = repo_handle.list_all();

    log_memory_usage();
    println!("finished main");
}
