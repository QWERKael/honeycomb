use std::error::Error;
use std::time::Duration;

use futures::stream;
// use rand::rngs::ThreadRng;
// use rand::Rng;
use tokio::time;
use tonic::transport::Channel;
use tonic::Request;
use async_std::io;

use communication::commander_client::CommanderClient;
use communication::{CmdRequest, CmdReply};

pub mod communication {
    tonic::include_proto!("communication");
}



async fn run_cmd_call(client: &mut CommanderClient<Channel>) -> Result<(), Box<dyn Error>> {
    let outbound = async_stream::stream! {
        let stdin = io::stdin();
        loop {
            let mut line = String::new();
            stdin.read_line(&mut line).await;
            let line = line.trim();
            println!("line is {:?}", line);

            let cmd_request = CmdRequest {
                using_db: "default".to_string(),
                cmd: 4,
                args: vec![line.as_bytes().to_vec()],
                // args: vec![b"aaa".to_vec()],
            };

            yield cmd_request;
        }
    };

    // let start = time::Instant::now();
    //
    // let outbound = async_stream::stream! {
    //     // let mut interval = time::interval(Duration::from_secs(1));
    //     //
    //     // while let time = interval.tick().await {
    //     //     let elapsed = time.duration_since(start);
    //     loop {
    //         let cmd_request = CmdRequest {
    //             cmd: 0,
    //             args: vec![b"aaa".to_vec()],
    //         };
    //
    //         yield cmd_request;
    //     }
    // };

    let response = client.cmd_call(Request::new(outbound)).await?;
    let mut inbound = response.into_inner();

    while let Some(cmd_reply) = inbound.message().await? {
        println!("Command Reply\n\tstatus: {}\n\tresult: {}", cmd_reply.status, String::from_utf8_lossy(&cmd_reply.result).to_string());
    }

    Ok(()
    )
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = CommanderClient::connect("http://[::1]:10000").await?;

    // println!("*** SIMPLE RPC ***");
    // let response = client
    //     .get_feature(Request::new(CmdRequest {
    //         cmd: 0,
    //         args: vec![b"k1".to_vec(), b"v1".to_vec()],
    //     }))
    //     .await?;
    // println!("RESPONSE = {:?}", response);
    //
    // println!("\n*** SERVER STREAMING ***");
    // print_features(&mut client).await?;
    //
    // println!("\n*** CLIENT STREAMING ***");
    // run_record_route(&mut client).await?;
    //
    // println!("\n*** BIDIRECTIONAL STREAMING ***");
    // run_route_chat(&mut client).await?;



    run_cmd_call(&mut client).await?;


    Ok(())
}