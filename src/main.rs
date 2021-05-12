extern crate rocksdb;

use rocksdb::{DB, WriteBatch, DBIterator};
use std::any::Any;
use std::fmt::Pointer;
use honeycomb::{Comb, RocksError};
use failure::Fail;
use std::collections::HashMap;
use async_std::io;

fn main() -> Result<(), RocksError> {
    // let mut db = DB::open_default("/Data/Rust/learn4rocksdb/rs.db").unwrap();
    let datadir = "rs.db";
    let mut conn = Comb::open(datadir)?;
    {
        {
            let mut db = conn.db.clone();
            let mut db = db.read()?;
            db.put(b"my key", "my value/我的值".as_bytes())?;
        }
        {
            let db = conn.db.clone();
            let mut db = db.read()?;
            match db.get(b"my key") {
                Ok(Some(value)) => println!("retrieved value: {}", String::from_utf8(value).unwrap()),
                Ok(None) => println!("value not found"),
                Err(e) => println!("operational problem encountered: {}", e),
            }
        }
        {
            let mut db = conn.db.clone();
            let mut db = db.read()?;
            db.delete(b"my key")?;
        }
    }

    let mut m: HashMap<&[u8], &[u8]> = HashMap::new();
    m.insert(b"key-1", b"val-1");
    m.insert(b"key-11", b"val-11");

    println!("use db0");
    conn.create_db("db0");
    println!("success");
    println!("set map");
    conn.set_map("db0", m);
    println!("success");

    println!("get val is {}", String::from_utf8(conn.get_val("db0", b"key-1").unwrap()).unwrap());


    // println!("using db: {:?}", conn.using_db);
    let m2 = conn.get_map("db0")?;
    for (k, v) in m2 {
        println!("{}: {}", String::from_utf8(k.to_vec()).unwrap(), String::from_utf8(v.to_vec()).unwrap());
    }

    let m3 = conn.get_map_utf8("db0")?;
    for (k, v) in m3 {
        println!("{}: {}", k, v)
    }

    // println!("using db: {:?}", conn.using_db);
    println!("all keys: {:?}", conn.get_keys("db0")?);
    println!("all keys utf8: {:?}", conn.get_keys_utf8("db0")?);
    conn.del_keys("db0", vec![b"key-1".to_vec(), b"key-11".to_vec()])?;
    println!("all keys: {:?}", conn.get_keys("db0")?);
    println!("all keys utf8: {:?}", conn.get_keys_utf8("db0")?);

    let v = conn.show_dbs()?;
    println!("{:?}", v);



    // {
    //     let mut db = conn.db.clone();
    //     let mut db = db.borrow();
    //     println!("creat cf");
    //     db.create_cf("test1", &conn.opts)?;
    //     println!("success");
    // }


    //
    // {
    //     let batch = WriteBatch::new();
    //     batch.put(b"a", b"1")?;
    //     batch.put(b"b", b"2")?;
    //     batch.put(b"c", b"3")?;
    //     batch.put(b"d", b"yes!")?;
    //     db.write(&batch)?;
    // }
    //
    // {
    //     let mut iter = db.iter();
    //     iter.seek(SeekKey::Start)?;
    //     loop {
    //         println!("{}", String::from_utf8_lossy(iter.key()));
    //         println!("{}", String::from_utf8_lossy(iter.value()));
    //         match iter.next() {
    //             Ok(ok) => if !ok { break; }
    //             _ => return Ok(())
    //         }
    //     }
    // }

    // println!("column family:");
    // for i in db.cf_names() {
    //     println!("{}", i);
    // }
//
    {
        // let cf1 = db.create_cf(("cf1", ColumnFamilyOptions::new())).unwrap();
        // let cf1 = db.cf_handle("cf1").unwrap();
        // println!("cf id: {:?}", cf1.);
        // let cf2 = db.create_cf(("cf2", ColumnFamilyOptions::new())).unwrap();
        // println!("cf id: {:?}", cf2.id());
    }
//     {
//         let cf1 = db.cf_handle("cf1").unwrap();
//         println!("cf id: {:?}", cf1.id());
//         db.put_cf(cf1, b"cf1-1", b"alex");
//         db.put_cf(cf1, b"cf1-2", b"bob");
//         db.put_cf(cf1, b"cf1-3", b"cindy");
//         println!("{}", db.get_cf(cf1, b"cf1-1").unwrap().unwrap().to_utf8().unwrap());
//         println!("{}", db.get_cf(cf1, b"cf1-2").unwrap().unwrap().to_utf8().unwrap());
//         println!("{}", db.get_cf(cf1, b"cf1-3").unwrap().unwrap().to_utf8().unwrap());
//
//         println!("column family:");
//         for i in db.cf_names() {
//             println!("{}", i);
//         }
//
//         let mut iter = db.iter_cf(cf1);
// //        let mut iter = db.iter();
//         iter.seek(SeekKey::Start)?;
//         println!("------------");
//         loop {
//             println!("{}: {}", String::from_utf8_lossy(iter.key()), String::from_utf8_lossy(iter.value()));
//             match iter.next() {
//                 Ok(ok) => if !ok { break; }
//                 _ => return Ok(())
//             }
//         }
//     }
//
//
// //    println!("{:?}", iter.key());
//
//
//     db.drop_cf("cf1");

    // drop(db);


    // println!("===============");
    // let sys_db = SysDB::open(datadir).unwrap();
    // sys_db.print_cfs();
    // sys_db.create_cf("cf1");
    // sys_db.print_cfs();
    // sys_db.create_cf("cf2");
    // sys_db.print_cfs();


    return Ok(());
}