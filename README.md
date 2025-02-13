# 🎟️ Ticket Lock API Microservice

## 📌 Overview

The **Ticket Lock API Microservice** ensures fair ticket reservations by temporarily locking tickets for a set duration. This prevents multiple users from booking the same ticket concurrently, reducing conflicts in high-demand ticketing systems. It leverages **Redis** for high-performance, in-memory ticket lock management and **Rust's Axum framework** for a scalable and efficient API.

## 🚀 Features

- **Atomic Ticket Locking:** Prevents multiple users from locking the same ticket.
- **Lock Expiration:** Automatically releases locks after a configurable time.
- **Check Ticket Status:** Query whether a ticket is locked or available.
- **Manual Lock Release:** Allows authorized users to release locked tickets.
- **High Performance:** Optimized using Redis, async Rust, and efficient memory handling.

---

## 🛠️ System Design

### **Architecture Overview**

- **Client Requests**: Users request a lock via an API call.
- **Ticket Lock Handling**: Requests are processed asynchronously.
- **Redis Storage**: Locks are stored with a TTL to auto-expire after the set duration.
- **Stateless API**: The service is stateless, relying on Redis for ticket state.

### **Component Diagram**

```bash
Client → API Gateway → Ticket Lock Service (Axum, Rust) → Redis
```

### **Tech Stack**

- **Rust** (Axum framework for web API)
- **Redis** (In-memory cache for ticket locks)
- **Tokio** (Async runtime for Rust)
- **Docker** (Containerization for deployment)
- **JWT Authentication** (For securing API endpoints)

### **Project Structure**

```bash
📂 ticket-lock-api
 ├── src
 │    ├── routes/
 │    │    ├── ticket.rs  # Ticket Lock API Routes
 │    ├── services/
 │    │    ├── redis.rs  # Redis Integration
 │    ├── config/
 │    │    ├── redis_config.rs  # Redis Configuration
 │    ├── main.rs  # Entry Point
 ├── .env  # Environment Variables
 ├── Cargo.toml  # Rust Dependencies
```

### **Environment Variables (`.env`)**

```env
REDIS_URL=redis://127.0.0.1:6379
```

---

## 📡 API Routes

### **🔒 Lock a Ticket**

**Endpoint:** `POST /lock`

- **Request Body:**

```json
{
    "ticket_id": "12345",
    "user_id": "user1",
    "duration": 60
}
```

- **Response:**

```json
{
    "message": "Ticket locked successfully"
}
```

### **🔍 Check Ticket Lock Status**

**Endpoint:** `GET /check/:ticket_id`

- **Response (if locked):**

```json
{
    "message": "{\"user_id\":\"user1\",\"locked_at\":1713141245}"
}
```

- **Response (if available):**

```json
{
    "message": "Ticket is available"
}
```

### **🔓 Release a Ticket**

**Endpoint:** `POST /release`

- **Request Body:**

```json
{
    "ticket_id": "12345",
    "user_id": "user1"
}
```

- **Response:**

```json
{
    "message": "Ticket released"
}
```

---

## ⚙️ How to Deploy

### **1️⃣ Start Redis with Docker**

```sh
docker run --name redis -p 6379:6379 -d redis
```

### **2️⃣ Build and Run the Microservice**

```sh
cargo build
cargo run
```

### **3️⃣ API Testing**

Use **Postman** or **cURL**:

```sh
curl -X POST http://localhost:3000/lock -H "Content-Type: application/json" -d '{"ticket_id":"12345", "user_id":"user1", "duration":60}'
```

---

## 🔄 How to Extend

### **1️⃣ Adjust Lock Expiry Time**

Modify the `EX` parameter in `lock_ticket` function inside `redis.rs`.

```rust
let set: bool = conn.set_ex(&key, lock_json, duration as usize).await?;
```

### **2️⃣ Implement Authentication**

- Integrate **JWT authentication** to restrict access to endpoints.
- Only authenticated users should lock/release tickets.

### **3️⃣ Add a Persistent Storage Layer**

- Store expired ticket locks in **PostgreSQL** or **MongoDB** for auditing.

---

## 📜 License

This project is **open-source** and available under the MIT License.

🚀 **Now you have a scalable, high-performance Ticket Lock Microservice!** Feel free to contribute or extend functionality. 🚀
