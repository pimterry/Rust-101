extern crate nickel;
extern crate rustache;

use std::io::net::ip::Ipv4Addr;
use nickel::{ Nickel, Request, Response, HttpRouter };
use rustache::{ HashBuilder, FileError };

fn main() {
    let mut server = Nickel::new();
    
    // /hello just returns "hello world"
    fn hello_world_handler(_: &Request, response: &mut Response) { 
        response.send("hello world"); 
    }

    server.get("/hello", hello_world_handler);

    // POSTing to /echo returns "You sent: [post body text]"
    fn echo_handler(request: &Request, response: &mut Response) {
        let input = &request.origin.body;
        response.send(format!("You sent: {}\n", input));
    }

    server.post("/echo", echo_handler);

    fn render_home(_: &Request, response: &mut Response) {
        let view_model = HashBuilder::new()
                         .insert_string("title", "Rustacular")
                         .insert_string("content", "Content!");

        let render_result = rustache::render_file("index.html", view_model);

        let result = match render_result {
            Ok(mut stream) => stream.read_to_string().unwrap(),
            Err(FileError(error)) => error,
            Err(_) => "Unknown error".to_string()
        };

        response.send(result);
    }

    server.get("/", render_home);

    server.listen(Ipv4Addr(0, 0, 0, 0), 6767);
}

