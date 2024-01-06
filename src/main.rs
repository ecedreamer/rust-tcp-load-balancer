use tokio::net::{TcpListener, TcpStream};
use tokio::io::{self};
use std::sync::atomic::{AtomicUsize, Ordering};


#[tokio::main]
async fn main() {
    // Replace these with your actual backend server addresses
    let backend_servers = vec!["127.0.0.1:8081", "127.0.0.1:8082", "127.0.0.1:8083"];
    let index = AtomicUsize::new(0);

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Load balancer listening on 127.0.0.1:8080");

    loop {
        match listener.accept().await {
            Ok((client_socket, _)) => {
                let backend_server = choose_backend(&backend_servers, &index);

                tokio::spawn(handle_client(client_socket, backend_server));
            }
            Err(e) => eprintln!("Error accepting client connection: {}", e),
        }
    }
}

fn choose_backend<'a>(backend_servers: &[&'a str], index: &AtomicUsize) -> &'a str {
    // Simple round-robin load balancing
    let current_index = index.fetch_add(1, Ordering::Relaxed) % backend_servers.len();
    backend_servers[current_index]
}

async fn handle_client(client_socket: TcpStream, backend_server: &str) {
    let backend_socket = TcpStream::connect(backend_server).await.unwrap();

    let (mut client_reader, mut client_writer) = io::split(client_socket);
    let (mut backend_reader, mut backend_writer) = io::split(backend_socket);

    let client_to_backend = io::copy(&mut client_reader, &mut backend_writer);
    let backend_to_client = io::copy(&mut backend_reader, &mut client_writer);

    tokio::select! {
        result = client_to_backend => {
            if let Err(e) = result {
                eprintln!("Error copying data from client to backend: {}", e);
            }
        }
        result = backend_to_client => {
            if let Err(e) = result {
                eprintln!("Error copying data from backend to client: {}", e);
            }
        }
    }
}
