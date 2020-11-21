use futures::future::{self, Ready};
use tarpc::context;

// This is the service definition. It looks a lot like a trait definition.
// It defines one RPC, hello, which takes one arg, name, and returns a String.
#[tarpc::service]
pub trait World {
    async fn hello(name: String) -> String;
}

// This is the type that implements the generated World trait. It is the business logic
// and is used to start the server.
#[derive(Clone)]
pub struct HelloServer;

impl World for HelloServer {
    // Each defined rpc generates two items in the trait, a fn that serves the RPC, and
    // an associated type representing the future output by the fn.

    type HelloFut = Ready<String>;

    fn hello(self, _: context::Context, name: String) -> Self::HelloFut {
       println!("Responding to hello");
       future::ready(format!("Hello, {}!", name))
    }
}
