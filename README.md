# Payment Processor - PrexCORE Challenge

![Rust](https://img.shields.io/badge/Rust-1.70.0-orange?logo=rust) ![Actix Web](https://img.shields.io/badge/Actix-Web-blue)

## Description

This project implements a REST microservice in Rust using Actix Web that works as a mini payment processor. It allows managing clients, crediting and debiting balances, querying balances, and persisting balances to files.

It is designed for card issuers and payment services, not for end customers.

---

## Technologies

- Rust
- Actix Web
- Tokio (asynchronous runtime)
- Serde (JSON serialization and deserialization)
- rust_decimal (decimal number handling)

---

## Endpoints

| Method | Endpoint                          | Description                                                                                     |
|--------|----------------------------------|-------------------------------------------------------------------------------------------------|
| POST   | `/client/new_client`              | Creates a new client. Requires: `client_name`, `birth_date` (format `YYYY-MM-DD`), unique `document_number`, and `country`. Returns generated unique client ID. |
| POST   | `/client/new_credit_transaction` | Credits balance to a client by ID. Receives `client_id` and `credit_amount`. Returns new balance. |
| POST   | `/client/new_debit_transaction`  | Debits balance from a client by ID. Receives `client_id` and `debit_amount`. Returns new balance. |
| POST   | `/client/store_balance`           | Persists all clients' balances to a file and resets in-memory balances to zero. The file is named with date and counter (`DDMMYYYY_COUNTER.DAT`). |
| GET    | `/client/client_balance/{id}`    | Returns info and current balance for the client with the specified ID.                          |

---

## Validation and Business Logic

- `document_number` must be unique; duplicate clients are not allowed.
- Credit and debit amounts are positive decimal numbers.
- Debits may result in negative balances (no minimum balance restriction).
- Clear errors with appropriate HTTP status codes (e.g., 404, 500) and descriptive response messages.

---

## Persistence

- Client data and balances are kept **in memory** during execution.
- Persistence to disk is triggered by calling **`/client/store_balance`**, which saves all balances in a file named:  
  ```
  1. DDMMYYYY_COUNTER.DAT
  ```
  for example: `01122023_10.DAT`.
- File format example:  
  ```
  1. ID_CLIENTE BALANCE
  2. ID_CLIENTE BALANCE
  ...
  ```
- After persistence, **all in-memory balances are reset to zero**.

---

## How to Run the Project

### Requirements

- Rust (recommended 1.70+)
- Cargo
- Postman for testing (collection included)

### Steps to Run

```bash
git clone <repository-url>
cd <project-name>
cargo run
```
The server runs by default at:
```
http://localhost:8080/api/v1
```
---

## How to Test

✅ Import the included Postman collection in the repo:
```
client_transactions.postman_collection.json
```
✅ Use the provided requests to:
- Create clients
- Credit or debit balances
- Query balances
- Generate balance files

---

## Tests
This project includes:

- ✅ **Unit tests** for core domain functions, validations, and business logic.
- ✅ **Integration tests** validating full endpoint behavior using Actix Web.

### Running Tests
```bash
cargo test
```
### Measuring Code Coverage
This project uses cargo-llvm-cov for test coverage measurement.

To generate and open an HTML coverage report:
```bash
cargo llvm-cov --open
```
This will open an interactive browser report showing coverage per module and function.
## Applied Best Practices
- Modular and readable code following **SOLID**, **KISS** and **DRY**.

- Use of **Tokio** for asynchronous high-performance runtime.

- **Serde** for safe and efficient JSON serialization and deserialization.

- Rigorous input validation to avoid inconsistencies.

- Consistent **HTTP status codes** and clear error messages.

- Extensive testing with unit and integration tests.

---

Thank you very much! 🚀
