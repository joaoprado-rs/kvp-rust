
# KVP Rust

KVP Rust is an API written in Rust without using any HTTP crate. The project is currently under development and serves as a foundation for building a multi-threaded HTTP server with enhanced routing capabilities.

## Features

- **Basic HTTP Server**: A basic implementation of an HTTP server that handles incoming TCP connections.
- **HTTP Request Parsing**: Parses HTTP requests and sends basic responses.

## Planned Enhancements

- **Thread Pool**: Implement a thread pool to handle multiple connections concurrently, improving performance and scalability.
- **Enhanced Routing**: Add support for more routes and dynamic route handling to serve different endpoints.
- **Improved Error Handling**: Provide more descriptive error messages and use appropriate HTTP status codes.
- **State Management**: Introduce shared state management to handle data across multiple connections.

## Current Structure

- A Rust HTTP server with no external dependencies for HTTP.
- Listens on port `7000` for TCP connections.
- Sends dynamic responses.

## How to Run

1. Clone the repository:
   
   ```bash
   git clone https://github.com/username/KVP-Rust.git
   cd KVP-Rust
