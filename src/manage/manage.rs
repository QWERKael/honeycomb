use rocksdb::{DB, Options, IteratorMode, WriteBatch};
use super::super::error::*;
use std::{collections::HashMap, fmt};
use std::sync::{Arc, RwLock};
use std::cell::RefCell;

// #[derive(Debug)]
// pub struct SysDB {
//     pub datadir: String,
//     pub db: DB,
//     pub cfs: Vec<String>,
// }
//
// impl SysDB {
//     pub fn open(datadir: &str) -> Result<SysDB, RocksError> {
//         let sys_dir = path::Path::new(datadir).join("sys");
//         let sys_dir = sys_dir.to_str().unwrap();
//         let mut sys_db: SysDB;
//         let mut opts = Options::default();
//         opts.create_if_missing(true);
//         // let mut cf_opts = ColumnFamilyOptions::new();
//         opts.set_merge_operator("concat", concat_merge, None);
//         match DB::open_cf(&opts, sys_dir, vec!["default"]) {
//             Ok(rocksDB) => {
//                 sys_db = SysDB {
//                     datadir: String::from(sys_dir),
//                     db: rocksDB,
//                     cfs: vec!["default".to_string()],
//                 };
//                 match sys_db.db.get(b"ColumnFamilies") {
//                     Ok(Some(value)) => {
//                         // let cfs = value.to_utf8().unwrap();
//                         let cfs = String::from_utf8(value).unwrap();
//                         let parts: Vec<String> = cfs.split(",").map(|s| s.trim().to_string()).collect();
//                         sys_db.cfs = parts;
//                         Ok(sys_db)
//                     }
//                     Ok(None) => {
//                         sys_db.db.put(b"ColumnFamilies", b"default");
//                         Ok(sys_db)
//                     }
//                     Err(e) => Err(RocksError::SysDB(e.into_string())),
//                 }
//             }
//             Err(e) => Err(RocksError::Connect(e.into_string()))
//         }
//     }
//
//     pub fn print_cfs(&self) {
//         match self.db.get(b"ColumnFamilies") {
//             Ok(Some(value)) => {
//                 // let cfs = value.to_utf8().unwrap();
//                 let cfs = String::from_utf8(value).unwrap();
//                 println!("column families :{}", cfs);
//             }
//             Ok(None) => println!("column families is none"),
//             Err(e) => println!("get column families error!"),
//         };
//     }
//
//     // pub fn create_cf(&self, cf_name: &str) -> Result<(), RocksError> {
//     //     if self.cfs.contains(&cf_name.to_string()) {
//     //         Err(RocksError::SysDB(format!("column family {} already exist", cf_name)))
//     //     } else {
//     //         let s = format!(",{}", cf_name);
//     //         self.db.merge(b"ColumnFamilies", s.as_bytes());
//     //         Ok(())
//     //     }
//     // }
//
//     pub fn create_cf(&self, cf_name: &str) -> Result<(), RocksError> {
//         if self.cfs.contains(&cf_name.to_string()) {
//             Err(RocksError::SysDB(format!("column family {} already exist", cf_name)))
//         } else {
//             let s = format!(",{}", cf_name);
//             self.db.merge(b"ColumnFamilies", s.as_bytes());
//             Ok(())
//         }
//     }
// }

pub struct Comb {
    pub datadir: String,
    pub opts: Options,
    pub db: Arc<RwLock<DB>>,
    // pub using_db: String,
}

impl fmt::Debug for Comb {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "pub struct Comb
    pub datadir: {},
", self.datadir)
    }
}

impl Comb {
    pub fn open(datadir: &str) -> Result<Comb, RocksError> {
        let mut opts = Options::default();
        // let cfs = DB::list_cf(&opts, &datadir)?;
        opts.create_if_missing(true);
        let cfs;
        if let Ok(vs) = DB::list_cf(&opts, &datadir) {
            cfs = vs
        } else {
            cfs = vec![String::from("default")]
        }
        match DB::open_cf(&opts, datadir, cfs) {
            // match DB::open_default(datadir) {
            Ok(db) => {
                let comb = Comb {
                    datadir: String::from(datadir),
                    opts,
                    db: Arc::new(RwLock::new(db)),
                    // using_db: "default".to_string(),
                };
                Ok(comb)
            }
            Err(e) => Err(e.into())
        }
    }

    pub fn show_dbs(&self) -> Result<Vec<String>, RocksError> {
        let v = DB::list_cf(&self.opts, self.datadir.clone())?;
        Ok(v)
    }

    // pub fn use_db(&'a mut self, name: &str) -> Result<(), RocksError> {
    //     self.using_db = Some(UsingDB {
    //         name: name.to_string(),
    //         cf_handle: match self.db.cf_handle(name) {
    //             Some(cf) => cf,
    //             None => return Err(RocksError::from(format!("获取不到列族: {}", name)))
    //         },
    //     });
    //     Ok(())
    // }

    pub fn create_db(&mut self, name: &str) -> Result<(), RocksError> {
        let dbs = self.show_dbs()?;
        if dbs.contains(&name.to_string()) {
            // self.using_db = name.to_string();
            Ok(())
        } else {
            // let mut db = self.db.clone();
            if let Ok(mut db) = self.db.try_write() {
                db.create_cf(name, &self.opts)?;
                Ok(())
            } else {
                Err(RocksError::from(format!("Can not change db")))
            }
        }
    }

    pub fn set_val(&self, using_db: &str, key: &[u8], val: &[u8]) -> Result<(), RocksError> {
        if let Some(cf) = self.db.read()?.cf_handle(using_db) {
            let db = self.db.clone();
            let db = db.read()?;
            db.put_cf(cf,key,val)?;
            Ok(())
        } else {
            Err(RocksError::from(format!("未指定列族")))
        }
    }

    pub fn set_map(&self, using_db: &str, map: HashMap<&[u8], &[u8]>) -> Result<(), RocksError> {
        if let Some(cf) = self.db.read()?.cf_handle(using_db) {
            let mut batch = WriteBatch::default();
            for (k, v) in map {
                batch.put_cf(cf, k, v)?;
            }
            self.db.read()?.write(batch)?;
            Ok(())
        } else {
            Err(RocksError::from(format!("未指定列族")))
        }
    }

    pub fn get_val(&self, using_db: &str, key: &[u8]) -> Result<Vec<u8>, RocksError> {
        if let Some(cf) = self.db.read()?.cf_handle(using_db) {
            let db = self.db.clone();
            let db = db.read()?;
            let snap = db.snapshot();
            match snap.get_cf(cf, key) {
                Ok(Some(value)) => Ok(value),
                Ok(None) => Ok(vec![]),
                Err(e) => Err(RocksError::from(e)),
            }
        } else {
            Err(RocksError::from(format!("未指定列族")))
        }
    }

    pub fn get_map(&self, using_db: &str) -> Result<HashMap<Box<[u8]>, Box<[u8]>>, RocksError> {
        if let Some(cf) = self.db.read()?.cf_handle(using_db) {
            let db = self.db.clone();
            let db = db.read()?;
            let snap = db.snapshot();
            let iter = snap.iterator_cf(cf, IteratorMode::Start)?;
            Ok(iter.fold(HashMap::new(), |mut m, (k, v)| {m.insert(k, v);m}))
        } else {
            Err(RocksError::from(format!("未指定列族")))
        }
    }

    pub fn get_map_utf8(&self, using_db: &str) -> Result<HashMap<String, String>, RocksError> {
        if let Some(cf) = self.db.read()?.cf_handle(using_db) {
            let db = self.db.clone();
            let db = db.read()?;
            let snap = db.snapshot();
            let iter = snap.iterator_cf(cf, IteratorMode::Start)?;
            Ok(iter.fold(HashMap::new(), |mut m, (k, v)| {m.insert(String::from_utf8_lossy(&k).to_string(), String::from_utf8_lossy(&v).to_string());m}))
        } else {
            Err(RocksError::from(format!("未指定列族")))
        }
    }

    pub fn del_keys(&self, using_db: &str, keys: Vec<Vec<u8>>) -> Result<(), RocksError> {
        if let Some(cf) = self.db.read()?.cf_handle(using_db) {
            let mut batch = WriteBatch::default();
            for key in keys {
                batch.delete_cf(cf, key)?;
            }
            self.db.read()?.write(batch)?;
            Ok(())
        } else {
            Err(RocksError::from(format!("未指定列族")))
        }
    }

    pub fn get_keys(&self, using_db: &str) -> Result<Vec<Vec<u8>>, RocksError> {
        if let Some(cf) = self.db.read()?.cf_handle(using_db) {
            let db = self.db.clone();
            let db = db.read()?;
            let snap = db.snapshot();
            let iter = snap.iterator_cf(cf, IteratorMode::Start)?;
            Ok(iter.fold(Vec::new(), |mut v, (k ,_)| {v.push(k.to_vec());v}))
        } else {
            Err(RocksError::from(format!("未指定列族")))
        }
    }

    pub fn get_keys_utf8(&self, using_db: &str) -> Result<Vec<String>, RocksError> {
        if let Some(cf) = self.db.read()?.cf_handle(using_db) {
            let db = self.db.clone();
            let db = db.read()?;
            let snap = db.snapshot();
            let iter = snap.iterator_cf(cf, IteratorMode::Start)?;
            Ok(iter.fold(Vec::new(), |mut v, (k ,_)| {v.push(String::from_utf8_lossy(&k).to_string());v}))
        } else {
            Err(RocksError::from(format!("未指定列族")))
        }
    }
}

// fn concat_merge(new_key: &[u8], existing_val: Option<&[u8]>,
//                 operands: &mut MergeOperands) -> Vec<u8> {

// fn concat_merge(_: &[u8], existing_val: Option<&[u8]>,
//                 operands: &mut MergeOperands) -> Option<Vec<u8>> {
//     let mut result: Vec<u8> = Vec::with_capacity(operands.size_hint().0);
//     existing_val.map(|v| {
//         for e in v {
//             result.push(*e)
//         }
//     });
//     for op in operands {
//         for e in op {
//             result.push(*e)
//         }
//     }
//     Some(result)
// }