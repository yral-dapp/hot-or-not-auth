use std::{collections::HashMap, sync::Arc};

use redb::{Database, ReadableTable, TableDefinition};
use tokio::task::spawn_blocking;

use super::KVStore;

const TABLE: TableDefinition<&str, &str> = TableDefinition::new("auth-kv"); 
const RAW_METADATA_TABLE: TableDefinition<&str, &[u8]> = TableDefinition::new("auth-kv-meta");

#[derive(Clone)]
pub struct MockKV(Arc<Database>);

impl MockKV {
    pub fn new() -> Result<Self, redb::Error> {
        let db = Database::create("./mock.db")?;
        let write_txn = db.begin_write()?;
        {
            write_txn.open_table(TABLE)?;
            write_txn.open_table(RAW_METADATA_TABLE)?;
        }
        write_txn.commit()?;
        Ok(Self(Arc::new(db)))
    }
}

impl KVStore for MockKV {
    async fn read_kv(&self, key_name: &str) -> Option<String> {
        let db = self.0.clone();
        let key = key_name.to_string();
        spawn_blocking(move || {
            let read_txn = db.begin_read()?;
            let value = {
                let table = read_txn.open_table(TABLE)?;
                let v = table.get(key.as_str())?;
                v.map(|ag| ag.value().to_string())
            };
            Ok::<_, redb::Error>(value)
        }).await.unwrap().unwrap()
    }

    async fn read_metadata(&self, key_name: &str) -> Option<HashMap<String, String>> {
        let db = self.0.clone();
        let key = key_name.to_string();
        spawn_blocking(move || {
            let read_txn = db.begin_read()?;
            let value = {
                let table = read_txn.open_table(RAW_METADATA_TABLE)?;
                let v = table.get(key.as_str())?;
                v.map(|ag| serde_json::from_slice(ag.value()).unwrap())
            };
            Ok::<_, redb::Error>(value)
        }).await.unwrap().unwrap()
    }

    async fn write_kv(&self, key_name: &str, value: &str, metadata: HashMap<&str, &str>) -> Option<String> {
        let db = self.0.clone();
        let key = key_name.to_string();
        let value = value.to_string();
        let meta_raw = serde_json::to_vec(&metadata).unwrap();
        spawn_blocking(move || {
            let write_txn = db.begin_write()?;
            let val;
            {
                let mut table = write_txn.open_table(TABLE)?;
                let res = table.insert(key.as_str(), value.as_str())?;
                val = res.map(|ag| ag.value().to_string());
                let mut meta = write_txn.open_table(RAW_METADATA_TABLE)?;
                meta.insert(key.as_str(), meta_raw.as_slice())?;
            }
            write_txn.commit()?;
            Ok::<_, redb::Error>(val)
        }).await.unwrap().unwrap()
    }

    async fn delete_kv(&self, key_name: &str) -> Option<String> {
        let db = self.0.clone();
        let key = key_name.to_string();
        spawn_blocking(move || {
            let write_txn = db.begin_write()?;
            let val;
            {
                let mut table = write_txn.open_table(TABLE)?;
                let res = table.remove(key.as_str())?;
                val = res.map(|ag| ag.value().to_string());
                let mut meta = write_txn.open_table(RAW_METADATA_TABLE)?;
                meta.remove(key.as_str())?;
            }
            write_txn.commit()?;
            Ok::<_, redb::Error>(val)
        }).await.unwrap().unwrap()
    }
}

