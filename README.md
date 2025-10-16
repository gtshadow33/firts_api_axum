# 🦀 First API Axum

A simple **CRUD API** in **Rust** using **Axum** to manage users.

[![Rust](https://img.shields.io/badge/Rust-1.72-blue?logo=rust)](https://www.rust-lang.org/) [![Axum](https://img.shields.io/badge/Axum-0.7.0-lightgrey?logo=rust)](https://docs.rs/axum/latest/axum/)

---

## 📖 Overview

This project is a **basic user CRUD API** that allows you to:

- List all users
- Get a user by ID
- Create a new user
- Update an existing user
- Delete a user

Users are stored in memory using a `HashMap` wrapped in `Arc<Mutex<_>>` for thread-safe access.

---

## 🛠 Technologies

- **Rust** – programming language  
- **Axum** – web framework  
- **Serde** – serialization & deserialization  
- **Tokio** – async runtime  
- **Tracing** – logging  

---
