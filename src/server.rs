use tarpc_interprocess::*;
use futures::prelude::*;
use interprocess::nonblocking::local_socket::*;
use std::io;
use tarpc::serde_transport::Transport;
use tarpc::{rpc::server::Channel, server};
use tokio_serde::formats::Bincode;
use tokio_util::compat::FuturesAsyncReadCompatExt;

#[tokio::main]
async fn main() -> io::Result<()> {
    let (_log, _guard) = create_stdout_logger(slog::Level::Debug).await;

    let incoming = LocalSocketListener::bind("/tmp/example.sock")
        .await
        .unwrap()
        .incoming()
        .map(|stream| match stream {
            Ok(stream) => {
                // `stream` implements `AsyncRead` + `AsyncWrite`
                println!("Incoming connection");
                Transport::from((stream.compat(), Bincode::default()))
            }
            Err(e) => panic!("{}", e),
        });

    server::new(server::Config::default())
        .incoming(incoming)
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
