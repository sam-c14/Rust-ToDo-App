# 🚀 Rust TaskMaster API

A high-performance, thread-safe CRUD Todo API built with **Rust** and **Actix-web**. This application demonstrates clean architecture, shared state management using `Mutex`, and robust JSON serialization with `Serde`.

---

## 🛠 Features

- **Blazing Fast**: Built on Actix-web, one of the fastest web frameworks in existence.
- **Thread-Safe State**: Utilizes `Mutex` for safe data access across multiple worker threads.
- **RESTful Design**: Pure JSON API following standard HTTP conventions.
- **Type-Safe**: Leveraging Rust's powerful type system to eliminate runtime errors.

---

## 🛰 API Endpoints

The API is grouped under the `/tasks` scope.

| Method | Endpoint | Description |
| :--- | :--- | :--- |
| `GET` | `/` | Health check & task summary |
| `GET` | `/tasks/` | Retrieve all tasks |
| `POST` | `/tasks/` | Create a new task |
| `GET` | `/tasks/{id}` | Get details of a specific task |
| `PATCH` | `/tasks/{id}` | Update an existing task |
| `DELETE` | `/tasks/{id}` | Remove a task |

---

## 📦 Request & Response Schema

### Standard Response Format
All responses are wrapped in a consistent JSON envelope:

```json
{
    "data": [...], 
    "status_code": 200,
    "message": "Welcome to task manager, You have 5 tasks"
}