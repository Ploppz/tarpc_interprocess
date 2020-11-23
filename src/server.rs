use tarpc_interprocess::*;
use futures::prelude::*;
use interprocess::nonblocking::local_socket::*;
use std::{
    io,
    net::IpAddr
};
use tarpc::serde_transport::Transport;
use tarpc::{rpc::server::Channel, server};
use tokio_serde::formats::Bincode;
use tokio_util::compat::FuturesAsyncReadCompatExt;

#[tokio::main]
async fn main() -> io::Result<()> {
    let (_log, _guard) = create_stdout_logger(slog::Level::Debug).await;

    let server_addr = (IpAddr::from([0, 0, 0, 0]), 5555);
    let mut listener = tarpc::serde_transport::tcp::listen(&server_addr, Bincode::default).await?;
    listener.config_mut().max_frame_length(4294967296);
    let listener = listener
        .map(|transport| {
            transport.unwrap()
        });
    server::new(server::Config::default())
        .incoming(listener)
        .map(|channel| {
            println!("Creating server");
            channel.respond_with(HelloServer.serve()).execute()
        })
        // Max 10 channels.
        .buffer_unordered(10)
        .for_each(|()| async {})
        .await;

    Ok(())
}
