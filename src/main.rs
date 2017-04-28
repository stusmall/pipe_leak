extern crate ws;

use ws::CloseCode;

fn main() {
    loop {
        println!("Building...");
        ws::connect("ws://127.0.0.1:8080", move |out| {
            move |msg| {
                println!("Got message: {}", msg);
                out.close(CloseCode::Normal)
            }
        }).unwrap();
    }
}
