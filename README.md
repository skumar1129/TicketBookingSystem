# Train Booking System

This repository contains two implementations of a train booking system: one in **C++** and one in **Rust**. Both codebases implement a console-based application for booking train tickets, storing data in a JSON file, and provide a foundation for managing users, vehicles, and bookings. The C++ version uses object-oriented programming with templates, while the Rust version leverages traits and safe memory management to achieve similar functionality.

## Project Overview

The train booking system allows users to:
- Input personal details (user ID, name, Aadhar card number).
- Book a train by providing train details (ID, name, source, destination).
- Save bookings to a JSON file (`db.json`) for persistence.
- Potentially cancel or view bookings (though these features are unimplemented placeholders).

Both implementations include:
- **Entities**: Data structures for `User`, `Train`, and `Vehicle`.
- **Services**: Logic for file I/O (saving/loading JSON data) and booking operations.
- **Main Program**: A console interface for user interaction and booking.

The C++ codebase uses templates and the `nlohmann/json` library, while the Rust codebase uses traits and the `serde`/`serde_json` crates, reflecting idiomatic practices for each language.


## Features

- **User Management**: Stores user details (ID, name, Aadhar card number).
- **Train Booking**: Creates a booking with train/vehicle details and saves it to `db.json`.
- **File I/O**: Persists data in JSON format, with a 2D seating arrangement (`seats` as a vector of user vectors).
- **Extensibility**: Includes placeholders for canceling and viewing bookings.
- **Conversion**: Supports converting a `Vehicle` to a `Train` (though used minimally).

### C++ Implementation
- Uses C++17 with templates for generic `FileIO` and `BookingService` classes.
- Relies on `nlohmann/json` for JSON serialization/deserialization.
- Stores data in `db.json` using `std::fstream`.
- Includes a `Makefile` for building with `g++`.

### Rust Implementation
- Uses Rust 2021 edition with traits for generic `FileIO` and `BookingService` functionality.
- Uses `serde` and `serde_json` for JSON handling.
- Manages file I/O with `std::fs` and explicit error handling via `Result`.
- Leverages `cargo` for building, with a `Makefile` wrapping `cargo` commands for consistency with the C++ project.
- Emphasizes memory safety and ownership, avoiding pointers used in C++ (e.g., `vector<Vehicle*>` in `User` is omitted as unused).

## Key Differences

| Feature                | C++ Implementation                          | Rust Implementation                          |
|------------------------|---------------------------------------------|---------------------------------------------|
| **Language Paradigm**  | Object-oriented with templates              | Functional with traits and ownership        |
| **JSON Library**       | `nlohmann/json`                            | `serde`/`serde_json`                        |
| **File I/O**           | `std::fstream` with minimal error handling  | `std::fs` with explicit `Result` handling   |
| **Input Handling**     | `std::cin` (single-word inputs)            | `std::io::stdin` (supports multi-word inputs) |
| **Safety**             | Manual memory management (e.g., pointers)   | Safe memory management (no pointers)        |
| **Build System**       | `Makefile` with `g++`                      | `cargo` with `Makefile` wrapper             |
| **Error Handling**     | Implicit (e.g., file open checks)          | Explicit with `Result` and `Option`         |

## Prerequisites

### C++
- **Compiler**: `g++` supporting C++17.
- **Library**: `nlohmann/json` (place `json.hpp` in `external/` directory).
- **Build Tool**: `make` for the `Makefile`.

### Rust
- **Rust Toolchain**: Install via `rustup` (https://rustup.rs/).
- **Dependencies**: Managed by `Cargo.toml` (requires `serde` and `serde_json`).
- **Build Tool**: `cargo` (included with Rust) and `make` for the `Makefile`.

## Building and Running

### C++
1. **Setup**:
   - Ensure `g++` and `make` are installed.
   - Place `nlohmann/json`’s `json.hpp` in the `external/` directory.
2. **Build**:
   ```bash
   cd ticketBookingSystemCpp
   make
   ```
3. **Run**:
```
./booking_system
```
And follow prompts for input

### Rust
1. **Setup**:
   - Ensure `cargo` is installed.

2. **Build**:
   ```
   cd ticketBookingSystemRust
   cargo build --release --bin main
   ```

3. **Run**:
   ```
   ./target/release/main
   ```
And follow prompts for input
