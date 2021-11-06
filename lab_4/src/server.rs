use futures::{future, prelude::*};
use service::Operand;
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
};
use clap::Parser;
use tarpc::{
    context,
    server::{self, incoming::Incoming, Channel},
    tokio_serde::formats::Json,
};
// This is the type that implements the generated World trait. It is the business logic
// and is used to start the server.
#[derive(Clone)]
struct Server(SocketAddr);

#[derive(Parser)]
struct Flags {
    /// Sets the port number to listen on.
    #[clap(long)]
    port: u16,
}

#[tarpc::server]
impl Operand for Server {

    async fn add(self, _: context::Context, items: String) -> String {
        format!("Result = {}", items.split_whitespace().map(|s| s.parse::<f64>().expect("parse error")).sum::<f64>())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
let flags = Flags::parse();

    let server_addr = (IpAddr::V4(Ipv4Addr::LOCALHOST), flags.port);
    println!("Server starts at {:?}", &server_addr);
    // JSON transport is provided by the json_transport tarpc module. It makes it easy
    // to start up a serde-powered json serialization strategy over TCP.
    let mut listener = tarpc::serde_transport::tcp::listen(&server_addr, Json::default).await?;
    listener.config_mut().max_frame_length(usize::MAX);
    listener
        // Ignore accept errors.
        .filter_map(|r| future::ready(r.ok()))
        .map(server::BaseChannel::with_defaults)
        .max_channels_per_key(1, |t| t.transport().peer_addr().unwrap().ip())
        // serve is generated by the service attribute. It takes as input any type implementing
        // the generated World trait.
        .map(|channel| {
            let server = Server(channel.transport().peer_addr().unwrap());
            channel.requests().execute(server.serve())
        })
        // Max 10 channels.
        .buffer_unordered(10)
        .for_each(|_| async {})
        .await;

    Ok(())
}