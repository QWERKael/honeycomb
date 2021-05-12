#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::pin::Pin;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::cell::RefCell;
use std::time::Instant;

use futures::{Stream, StreamExt};
use tokio::sync::mpsc;
use tonic::transport::Server;
use tonic::{Request, Response, Status, IntoRequest};


use communication::commander_server::{Commander, CommanderServer};
use communication::{CmdRequest, CmdReply, cmd_reply::*, cmd_request::*};

// use honeycomb::communication::commander_server::{Commander, CommanderServer};
// use honeycomb::communication::{CmdRequest, CmdReply, cmd_reply::*, cmd_request::*};

use honeycomb::{Comb, RocksError};
use rocksdb::DB;
use std::net::SocketAddr;

lazy_static! {
    // static ref COMB: Arc<RwLock<Comb>> = Arc::new(RwLock::new(Comb::open("/Data/Rust/honeycomb/rs.db").unwrap()));
    static ref COMB: Arc<Comb> = Arc::new(Comb::open("/Data/Rust/honeycomb/rs.db").unwrap());
    // static ref CFMAP: Arc<RwLock<HashMap<SocketAddr, String>>> = Arc::new(RwLock::new(HashMap::new()));
}

pub mod communication {
    tonic::include_proto!("communication");
}

#[derive(Debug)]
pub struct CommanderService {
    // cf: Arc<RwLock<HashMap<SocketAddr, String>>>,
    // cf: Arc<RwLock<String>>,
    // sessions: HashMap<>,
}


fn execute(cmd_request: &CmdRequest) -> CmdReply {
    // let using_db = "default".to_string();
    let mut comb: Arc<Comb> = (*COMB).clone();
    let using_db = cmd_request.using_db.clone();
    let cmd = cmd_request.cmd.clone();
    let args = cmd_request.args.clone();

    println!("cmd is: {}\nargs are: {:?}", cmd, String::from_utf8_lossy(&args[0]).to_string());

    let mut status = ExeState::Ok as i32;
    let mut result = "执行成功".as_bytes().to_vec();

    // match cmd as CmdType {
    match cmd {
        // CmdType::Set
        0 => {
            let mut db = comb.db.clone();
            let mut db = db.read().unwrap();
            let key = args[0].clone();
            let val = args[1].clone();
            db.put(key, val);
            status = ExeState::Ok as i32;
            result = "插入成功".as_bytes().to_vec();
        }
        // CmdType::Get
        1 => {
            if let Ok(val) = comb.get_val(using_db.as_str(), &args[0]) {
                status = ExeState::Ok as i32;
                result = val;
            } else {
                status = ExeState::Err as i32;
                result = "获取失败".as_bytes().to_vec();
            }
        }
        // CmdType::Delete
        2 => {
            if let Ok(ok) = comb.del_keys(using_db.as_str(), args) {
                status = ExeState::Ok as i32;
                result = "删除成功".as_bytes().to_vec();
            } else {
                status = ExeState::Err as i32;
                result = "删除失败".as_bytes().to_vec();
            }
        }
        // CmdType::CreateDB
        3 => {
            if let Ok(ok) = comb.create_db(std::str::from_utf8(&args[0]).unwrap()) {
                status = ExeState::Ok as i32;
                result = "创建成功".as_bytes().to_vec();
            } else {
                status = ExeState::Err as i32;
                result = "创建失败".as_bytes().to_vec();
            }
        }
        // CmdType::ShowDb
        4 => {
            if let Ok(dbs) = comb.show_dbs() {
                status = ExeState::Ok as i32;
                result = dbs.join("\n").as_bytes().to_vec();
            } else {
                status = ExeState::Err as i32;
                result = "获取db列表失败".as_bytes().to_vec();
            }
        }
        _ => {
            status = ExeState::Err as i32;
            result = "无法识别的命令".as_bytes().to_vec();
        }
    }

    CmdReply {
        status,
        result,
    }
}


#[tonic::async_trait]
impl Commander for CommanderService {
    type CmdCallStream =
    Pin<Box<dyn Stream<Item=Result<CmdReply, Status>> + Send + Sync + 'static>>;

    async fn cmd_call(
        &self,
        request: Request<tonic::Streaming<CmdRequest>>,
    ) -> Result<Response<Self::CmdCallStream>, Status> {
        // 获取元数据
        // let m = request.metadata();
        // println!("metadata is: {:?}", m);

        // let (addr, cf )= if let Some(addr) = request.remote_addr() {
        //     println!("Socket Addr: {:?}", addr);
        //     let cf_map_reader: Arc<RwLock<HashMap<SocketAddr, String>>> = (*CFMAP).clone();
        //     let cf_map_reader: RwLockReadGuard<HashMap<SocketAddr, String>> = cf_map_reader.read().unwrap();
        //     if let Some(cf) = cf_map_reader.get(&addr) {
        //         let cf = cf.clone();
        //         (addr, Some(cf))
        //     } else {
        //         println!("未找到map值");
        //         println!("设置map默认值");
        //         (addr, None)
        //     }
        // } else {
        //     println!("未找到地址");
        //     panic!()
        // };
        //
        // let cf = if let Some(cf) = cf {
        //     cf
        // } else {
        //     let mut cf_map_writer: Arc<RwLock<HashMap<SocketAddr, String>>> = (*CFMAP).clone();
        //     let mut cf_map_writer: RwLockWriteGuard<HashMap<SocketAddr, String>> = cf_map_writer.write().unwrap();
        //     println!("设置writer");
        //     cf_map_writer.insert(addr, "default".to_string());
        //     "default".to_string()
        // };
        //
        // println!("获取到cf: {:?}", cf);
        //
        // {
        //     // 查看CFMAP
        //     let cf_map: Arc<RwLock<HashMap<SocketAddr, String>>> = (*CFMAP).clone();
        //     let cf_map = cf_map.read().unwrap();
        //     println!("all cf map keys: {:?}", cf_map.keys());
        // }

        println!("command call");

        let mut stream = request.into_inner();

        let output = async_stream::try_stream! {
            while let Some(cmd_request) = stream.next().await {
                let cmd_request = cmd_request?;
                let cmd_reply = execute(&cmd_request);
                yield cmd_reply
            }
        };

        Ok(Response::new(Box::pin(output) as Self::CmdCallStream))
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:10000".parse().unwrap();

    println!("Honeycomb listening on: {}", addr);

    let datadir = "rs.db";
    // let mut comb = Comb::open(datadir).expect("Open database error");

    let commander = CommanderService {
        // cf: Arc::new(RwLock::new(HashMap::new())),
    };

    // 初始化comb
    let comb: Arc<Comb> = (*COMB).clone();
    if let Ok(dbs) = comb.show_dbs() {
        println!("{:?}", dbs);
    } else {
        println!("无法获取数据库列表")
    }

    // {
    //     // 初始化cf_map
    //     let cf_map: Arc<RwLock<HashMap<SocketAddr, String>>> = (*CFMAP).clone();
    //     let cf_map = cf_map.read().unwrap();
    //     println!("all cf map keys: {:?}", cf_map.keys());
    // }

    let svc = CommanderServer::new(commander);

    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}