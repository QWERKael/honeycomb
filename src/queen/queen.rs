#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::pin::Pin;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::cell::RefCell;

use std::time::Instant;

use futures::{Stream, StreamExt, SinkExt};
use tokio::sync::mpsc;
use tonic::transport::Server;
use tonic::{Request, Response, Status, IntoRequest};


use dance::commander_server::{Commander, CommanderServer};
use dance::{CmdRequest, CmdReply, cmd_reply::*, cmd_request::*};

use honeycomb::{Comb, RocksError};
use rocksdb::DB;
use std::net::SocketAddr;

lazy_static! {
   static ref COMB: Arc<RwLock<Comb>> = Arc::new(RwLock::new(Comb::open("rs.db").unwrap()));
   }

pub mod dance {
    tonic::include_proto!("dance");
}

#[derive(Debug)]
pub struct CommanderService {}


fn execute(cmd_request: &CmdRequest) -> CmdReply {
    let mut comb: Arc<RwLock<Comb>> = (*COMB).clone();
    let using_db = cmd_request.using_db.clone();
    let cmd = cmd_request.cmd.clone();
    let args: Vec<std::vec::Vec<u8>> = cmd_request.args.clone();

    {
        let args = args.clone();
        println!("cmd is: {}", cmd);
        if args.len() > 0 {
            print!("args are: ");
            for arg in args {
                print!(" ");
                println!("{:?}", String::from_utf8_lossy(&arg).to_string());
            };
        };
        println!();
    }

    let mut status = ExeState::Ok as i32;
    let mut message = "执行成功".to_string();
    let mut results:Vec<Vec<u8>> = vec![vec![]];

    // match cmd as CmdType {
    match cmd {
        // CmdType::Set
        0 => {
            if let Ok(val) = comb.read().unwrap().set_val(using_db.as_str(),&args[0],&args[0]) {
                status = ExeState::Ok as i32;
                message = "插入成功".to_string();
            } else {
                status = ExeState::Err as i32;
                message = "插入失败".to_string();
            }
        }
        // CmdType::Get
        1 => {
            if let Ok(val) = comb.read().unwrap().get_val(using_db.as_str(), &args[0]) {
                status = ExeState::Ok as i32;
                message = "获取成功".to_string();
                results = vec![val];
            } else {
                status = ExeState::Err as i32;
                message = "获取失败".to_string();
            }
        }
        // CmdType::Delete
        2 => {
            if let Ok(ok) = comb.read().unwrap().del_keys(using_db.as_str(), args) {
                status = ExeState::Ok as i32;
                message = "删除成功".to_string();
            } else {
                status = ExeState::Err as i32;
                message = "删除失败".to_string();
            }
        }
        // CmdType::CreateDB
        3 => {
            if let Ok(ok) = comb.write().unwrap().create_db(std::str::from_utf8(&args[0]).unwrap()) {
                status = ExeState::Ok as i32;
                message = "创建成功".to_string();
            } else {
                status = ExeState::Err as i32;
                message = "创建失败".to_string();
            }
        }
        // CmdType::ShowDb
        4 => {
            if let Ok(dbs) = comb.read().unwrap().show_dbs() {
                status = ExeState::Ok as i32;
                message = "获取db列表成功".to_string();
                results = dbs.iter().map(|s| s.as_bytes().to_vec()).collect::<Vec<Vec<u8>>>();
            } else {
                status = ExeState::Err as i32;
                message = "获取db列表失败".to_string();
            }
        }
        // CmdType::Keys
        5 => {
            if let Ok(keys) = comb.read().unwrap().get_keys(using_db.as_str()) {
                status = ExeState::Ok as i32;
                message = "获取db列表成功".to_string();
                results = keys;
                // result = keys.join("\n").as_bytes().to_vec();
            } else {
                status = ExeState::Err as i32;
                message = "获取keys列表失败".to_string();
            }
        }
        _ => {
            status = ExeState::Err as i32;
            message = "无法识别的命令".to_string();
        }
    }

    CmdReply {
        status,
        message,
        results,
    }
}


#[tonic::async_trait]
impl Commander for CommanderService {
    async fn cmd_call(
        &self,
        request: Request<CmdRequest>,
    ) -> Result<Response<CmdReply>, Status> {
        // 获取元数据
        // let m = request.metadata();
        // println!("metadata is: {:?}", m);

        println!("command call");
        let cmd_request = request.into_inner();
        println!("{:?}", cmd_request);
        let cmd_reply = execute(&cmd_request);

        Ok(Response::new(cmd_reply))
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:10000".parse().unwrap();

    println!("Honeycomb listening on: {}", addr);

    let datadir = "rs.db";

    let commander = CommanderService {};


    // 初始化comb
    let comb: Arc<RwLock<Comb>> = (*COMB).clone();
    {
        comb.write().unwrap().create_db("db0");
    }
    if let Ok(dbs) = comb.read().unwrap().show_dbs() {
        println!("{:?}", dbs);
    } else {
        println!("无法获取数据库列表")
    }

    let svc = CommanderServer::new(commander);

    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}