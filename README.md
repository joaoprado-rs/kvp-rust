KVP Rust
This is a server written in Rust without any HTTP crate. The project is currently under development and serves as a foundation for building a multi-threaded HTTP server with enhanced routing capabilities.

Features
Basic HTTP server implementation.
Handles incoming TCP connections.
Parses HTTP requests and sends basic responses.
Planned Enhancements
Thread Pool:
Implement a thread pool to handle multiple connections concurrently, improving performance and scalability.

Enhanced Routing:
Add support for more routes and dynamic route handling to serve different endpoints.

Improved Error Handling:
Provide more descriptive error messages and proper HTTP status codes.

State Management:
Introduce shared state management for handling data across multiple connections.

Current Structure
How to Run
Clone the repository:

Build and run the project:

The server will start listening on 127.0.0.1:7000.

Example Usage
Currently, the server is in its early stages and does not support advanced routing. You can test it by sending a basic HTTP request using tools like curl or Postman.

Contributing
Contributions are welcome! If you'd like to contribute, please fork the repository and submit a pull request. Suggestions for improvements are also appreciated.

License
This project is licensed under the MIT License. See the LICENSE file for details.