use crate::script::Script::*;
use crate::script::{disk_plugin, CollabPersistenceTest};

#[test]
fn delete_single_doc_test() {
  let mut test = CollabPersistenceTest::new();
  let doc_id = "1".to_string();
  test.run_scripts(vec![
    CreateDocumentWithDiskPlugin {
      id: doc_id.clone(),
      plugin: disk_plugin(test.uid),
    },
    AssertNumOfDocuments { expected: 1 },
    DeleteDocument { id: doc_id },
    AssertNumOfDocuments { expected: 0 },
  ]);
}
#[test]
fn delete_multiple_docs_test() {
  let mut test = CollabPersistenceTest::new();
  let disk_plugin = disk_plugin(test.uid);
  test.run_scripts(vec![
    CreateDocumentWithDiskPlugin {
      id: "1".to_string(),
      plugin: disk_plugin.clone(),
    },
    CreateDocumentWithDiskPlugin {
      id: "2".to_string(),
      plugin: disk_plugin.clone(),
    },
    CreateDocumentWithDiskPlugin {
      id: "3".to_string(),
      plugin: disk_plugin,
    },
    DeleteDocument {
      id: "1".to_string(),
    },
    DeleteDocument {
      id: "2".to_string(),
    },
    AssertNumOfDocuments { expected: 1 },
  ]);
}
