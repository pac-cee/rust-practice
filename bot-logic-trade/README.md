# Bot Logic Trade - Polyglot Matching Engine Microservices

This project is a scalable, polyglot backend for a trading platform using microservices in Go, Java, Node.js, Rust, and C++. Each language implements a matching engine microservice, with Redis as a shared cache and event-driven stubs for Kafka/RabbitMQ integration.

## Features
- **Polyglot Microservices:** The project demonstrates the implementation of matching engines in multiple programming languages (Go, Java, Node.js, Rust, and C++). Each microservice is self-contained and showcases the idiomatic use of its respective language.
- **Unified API:** All services expose the same set of RESTful endpoints (`/order`, `/orderbook`, `/health`) to ensure consistency and make it easy to integrate or benchmark across different implementations.
- **Concurrency & Memory Management:** Each service employs concurrency mechanisms suitable for its language:
  - **Go:** Uses mutexes and atomic operations for thread safety.
  - **Java:** Relies on synchronized blocks and thread-safe collections.
  - **Node.js:** Leverages its single-threaded event loop for atomicity.
  - **Rust:** Utilizes `Mutex` and other thread-safe primitives.
  - **C++:** Implements mutexes for thread-safe operations.
- **Redis Caching:** Redis is used as a shared in-memory database to store orderbooks and order states. This ensures high performance and persistence across all services.
- **Event-Driven Stubs:** Stubs for Kafka/RabbitMQ are included to facilitate future integration with event-driven architectures, enabling features like settlement workflows or notifications.
- **Docker Compose:** A single `docker-compose.yml` file orchestrates all services and Redis, simplifying deployment and testing.
- **Comprehensive Documentation:** Each service includes inline comments and endpoint documentation to help developers understand the implementation and extend the functionality.
