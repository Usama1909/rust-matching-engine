# Rust Matching Engine

A multithreaded order matching engine built in Rust from scratch.

## Features
- Concurrent order processing using threads
- Shared order book with Arc<Mutex<>> for thread safety
- MPSC channels for order delivery pipeline
- Separate bid/ask books with price-time priority matching
- Partial fill support

## Architecture
- Thread 1: Order sender — sends orders through a channel
- Thread 2: Matching engine — receives orders and matches them in real time

## How to Run
cargo run

## Tech Stack
- Rust
- std::thread — concurrency
- std::sync::Arc, Mutex — shared state
- std::sync::mpsc — message passing