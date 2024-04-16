use assert_json_diff::assert_json_include;
use collab_database::rows::CreateRowParams;
use collab_plugins::CollabKVDB;
use serde_json::json;

use crate::database_test::helper::{
  create_database_with_db, restore_database_from_db, DatabaseTest,
};
use crate::helper::unzip_history_database_db;

#[tokio::test]
async fn restore_row_from_disk_test() {
  let (db, database_test) = create_database_with_db(1, "1").await;
  let row_1 = CreateRowParams {
    id: 1.into(),
    ..Default::default()
  };
  let row_2 = CreateRowParams {
    id: 2.into(),
    ..Default::default()
  };
  database_test.create_row(row_1.clone()).unwrap();
  database_test.create_row(row_2.clone()).unwrap();
  drop(database_test);

  let database_test = restore_database_from_db(1, "1", db);
  let rows = database_test.get_rows_for_view("v1");
  assert_eq!(rows.len(), 2);

  assert!(rows.iter().any(|row| row.id == row_1.id));
  assert!(rows.iter().any(|row| row.id == row_2.id));
}

#[tokio::test]
async fn restore_from_disk_test() {
  let (db, database_test) = create_database_with_db(1, "1").await;
  assert_database_eq(database_test);

  // Restore from disk
  let database_test = restore_database_from_db(1, "1", db);
  assert_database_eq(database_test);
}

#[tokio::test]
async fn restore_from_disk_with_different_database_id_test() {
  let (db, _) = create_database_with_db(1, "1").await;
  let database_test = restore_database_from_db(1, "1", db);

  assert_database_eq(database_test);
}

#[tokio::test]
async fn restore_from_disk_with_different_uid_test() {
  let (db, _) = create_database_with_db(1, "1").await;
  let database_test = restore_database_from_db(1, "1", db);

  assert_database_eq(database_test);
}

fn assert_database_eq(database_test: DatabaseTest) {
  let expected = json!( {
    "fields": [],
    "inline_view_id": "v1",
    "rows": [],
    "views": [
      {
        "database_id": "1",
        "field_orders": [],
        "filters": [],
        "group_settings": [],
        "id": "v1",
        "layout": 0,
        "layout_settings": {},
        "row_orders": [],
        "sorts": []
      }
    ]
  });

  assert_json_include!(
    expected: expected,
    actual: database_test.to_json_value()
  );
}

const HISTORY_DOCUMENT_020: &str = "020_database";

#[tokio::test]
async fn open_020_history_database_test() {
  let (_cleaner, db_path) = unzip_history_database_db(HISTORY_DOCUMENT_020).unwrap();
  let db = std::sync::Arc::new(CollabKVDB::open(db_path).unwrap());
  let database_test = restore_database_from_db(
    221439819971039232,
    "c0e69740-49f0-4790-a488-702e2750ba8d",
    db,
  );
  let actual = database_test.to_json_value();

  let expected = json!({
    "fields": [
      {
        "field_type": 0,
        "id": "E_50ji",
        "is_primary": true,
        "name": "Name",
        "type_options": {
          "0": {
            "data": ""
          }
        }
      },
      {
        "field_type": 3,
        "id": "8tbGTb",
        "is_primary": false,
        "name": "Type",
        "type_options": {
          "3": {
            "content": "{\"options\":[{\"id\":\"jydv\",\"name\":\"3\",\"color\":\"LightPink\"},{\"id\":\"F2ew\",\"name\":\"2\",\"color\":\"Pink\"},{\"id\":\"hUJE\",\"name\":\"1\",\"color\":\"Purple\"}],\"disable_color\":false}"
          }
        }
      },
      {
        "field_type": 5,
        "id": "e-5TiR",
        "is_primary": false,
        "name": "Done",
        "type_options": {
          "5": {
            "is_selected": false
          }
        }
      },
      {
        "field_type": 1,
        "id": "QfCqmc",
        "is_primary": false,
        "name": "Text",
        "type_options": {
          "0": {
            "data": "",
            "format": 0,
            "name": "Number",
            "scale": 0,
            "symbol": "RUB"
          },
          "1": {
            "format": 1,
            "name": "Number",
            "scale": 0,
            "symbol": "RUB"
          }
        }
      },
      {
        "field_type": 6,
        "id": "vdCF8I",
        "is_primary": false,
        "name": "Text",
        "type_options": {
          "0": {
            "content": "",
            "data": "",
            "url": ""
          },
          "6": {
            "content": "",
            "url": ""
          }
        }
      },
      {
        "field_type": 8,
        "id": "9U02fU",
        "is_primary": false,
        "name": "Text",
        "type_options": {
          "0": {
            "data": "",
            "date_format": 3,
            "field_type": 8,
            "time_format": 0,
            "timezone_id": ""
          },
          "8": {
            "date_format": 3,
            "field_type": 8,
            "time_format": 0,
            "timezone_id": ""
          }
        }
      }
    ],
    "inline_view_id": "b44b2906-4508-4532-ad9e-2cf33ceae304",
    "rows": [
      {
        "cells": {
          "8tbGTb": {
            "created_at": 1690639663,
            "data": "hUJE",
            "field_type": 3,
            "last_modified": 1690639663
          },
          "E_50ji": {
            "created_at": 1690639669,
            "data": "1",
            "field_type": 0,
            "last_modified": 1690639669
          },
          "QfCqmc": {
            "created_at": 1690639678,
            "data": "$1",
            "field_type": 1,
            "last_modified": 1690639678
          },
          "e-5TiR": {
            "created_at": 1690639660,
            "data": "Yes",
            "field_type": 5,
            "last_modified": 1690639660
          }
        },
        "height": 60,
        "id": "bbd404d8-1319-4e4d-84fe-1052c57fe3e7",
        "created_at": 1690639659,
        "modified_at": 1690639678,
        "visibility": true
      },
      {
        "cells": {
          "8tbGTb": {
            "created_at": 1690639665,
            "data": "F2ew",
            "field_type": 3,
            "last_modified": 1690639665
          },
          "E_50ji": {
            "created_at": 1690639669,
            "data": "2",
            "field_type": 0,
            "last_modified": 1690639669
          },
          "QfCqmc": {
            "created_at": 1690639679,
            "data": "$2",
            "field_type": 1,
            "last_modified": 1690639679
          },
          "e-5TiR": {
            "created_at": 1690639661,
            "data": "Yes",
            "field_type": 5,
            "last_modified": 1690639661
          }
        },
        "height": 60,
        "id": "bcfe322e-6272-4ed3-a57e-09645ec1073a",
        "created_at": 1690639659,
        "modified_at": 1690639679,
        "visibility": true
      },
      {
        "cells": {
          "8tbGTb": {
            "created_at": 1690639667,
            "data": "jydv",
            "field_type": 3,
            "last_modified": 1690639667
          },
          "E_50ji": {
            "created_at": 1690639670,
            "data": "3",
            "field_type": 0,
            "last_modified": 1690639670
          },
          "QfCqmc": {
            "created_at": 1690639679,
            "data": "$3",
            "field_type": 1,
            "last_modified": 1690639679
          },
          "e-5TiR": {
            "created_at": 1690639661,
            "data": "Yes",
            "field_type": 5,
            "last_modified": 1690639661
          }
        },
        "height": 60,
        "id": "5d4418d2-621a-4ac5-ad05-e2c6fcc1bc79",
        "created_at": 1690639659,
        "modified_at": 1690639679,
        "visibility": true
      }
    ],
    "views": [
      {
        "created_at": 1690639659,
        "database_id": "c0e69740-49f0-4790-a488-702e2750ba8d",
        "field_orders": [
          {
            "id": "E_50ji"
          },
          {
            "id": "8tbGTb"
          },
          {
            "id": "e-5TiR"
          },
          {
            "id": "QfCqmc"
          },
          {
            "id": "vdCF8I"
          },
          {
            "id": "9U02fU"
          }
        ],
        "filters": [
          {
            "condition": 2,
            "content": "",
            "field_id": "E_50ji",
            "id": "OWu470",
            "ty": 0
          }
        ],
        "group_settings": [],
        "id": "b44b2906-4508-4532-ad9e-2cf33ceae304",
        "layout": 0,
        "layout_settings": {},
        "modified_at": 1690639708,
        "name": "Untitled",
        "row_orders": [
          {
            "height": 60,
            "id": "bbd404d8-1319-4e4d-84fe-1052c57fe3e7"
          },
          {
            "height": 60,
            "id": "bcfe322e-6272-4ed3-a57e-09645ec1073a"
          },
          {
            "height": 60,
            "id": "5d4418d2-621a-4ac5-ad05-e2c6fcc1bc79"
          }
        ],
        "sorts": [
            {
              "condition": 0,
              "field_id": "E_50ji",
              "id": "s:4SJjUs",
              "ty": 0
            }
          ],
          "field_settings": {}
      }
    ]
  });

  assert_json_include!(expected: expected, actual: actual);
}
