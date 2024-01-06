## Simple Rust TCP Load Balancer with Tokio
This is a simple TCP load balancer implemented in Rust using the Tokio asynchronous runtime. The load balancer distributes incoming connections in a round-robin fashion among specified backend servers.

## Features
- Round-Robin Load Balancing: Incoming connections are evenly distributed across the configured backend servers.
- Asynchronous I/O: Utilizes the Tokio asynchronous runtime for efficient non-blocking I/O operations.
- Thread Safety: Uses AtomicUsize for thread-safe round-robin indexing.
## Usage
1. Clone the repository:

```bash
git clone https://github.com/ecedreamer/rust-tcp-load-balancer.git
cd rust-tcp-load-balancer
```

2. Update the backend server addresses in the main function:

```rust
let backend_servers = vec!["127.0.0.1:8081", "127.0.0.1:8082", "127.0.0.1:8083"];
```

3. Build and run the load balancer:

```bash
cargo run --release
```
4. The load balancer will be listening on 127.0.0.1:8080. You can connect to it, and it will distribute the connections among the specified backend servers.

## Configuration
Adjust the configuration variables in the main function according to your requirements:

- backend_servers: List of backend server addresses.
- listener_address: Address on which the load balancer listens for incoming connections.
## Notes
- This is a simple demonstration and may not be suitable for production use without additional features, error handling, and security considerations.
- Ensure that backend servers are running and accessible from the load balancer.

Feel free to customize and extend the load balancer based on your specific needs!