# Actix Web Project

This project is built using the [Actix Web](https://actix.rs/) framework version 4.9.0, leveraging Rust 1.84.0 for high-performance web development.

## Features

- High-performance HTTP server built with Actix Web.
- Modular and scalable design.
- Easy integration with third-party libraries and Rust's strong type system.

## Dependencies

### Major Dependencies
- **Actix Web 4.9.0**: For creating fast and reliable web services.

You can find the complete list of dependencies in the `Cargo.toml` file.

## Prerequisites

- Install Rust (version 1.84.0 or later): [Rust Installation Guide](https://www.rust-lang.org/tools/install)
- Install `cargo`: Comes with Rust installation.
- Make sure `actix-web` is added as a dependency in your `Cargo.toml` file.

## Getting Started

### 1. Clone the Repository:
```bash
git clone https://github.com/ImKairat/ikigai_back_end.git
cd ikigai_back_end
```

### 2. Build the Project:
```bash
cargo build
```

### 3. Run the Server:
```bash
cargo run
```

Once started, the server will run locally, and you can access it in your browser or using a tool like `curl`.

### 4. Testing the Application:
Run the following command to execute the test suite:
```bash
cargo test
```

## Project Structure

Here’s a high-level overview of the project structure:
```plaintext
. ├── src
  │ ├── main.rs # Entry point of the application 
  │ └── routes/ # Folder for route-related logic 
  ├── Cargo.toml # Dependency and project configuration
