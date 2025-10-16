<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>First API Axum</title>
    <style>
        body { font-family: Arial, sans-serif; line-height: 1.6; padding: 20px; background-color: #f9f9f9; }
        h1, h2, h3 { color: #333; }
        code { background: #eee; padding: 2px 6px; border-radius: 4px; }
        pre { background: #eee; padding: 10px; border-radius: 4px; overflow-x: auto; }
        table { border-collapse: collapse; width: 100%; margin-bottom: 20px; }
        th, td { border: 1px solid #ccc; padding: 8px; text-align: left; }
        th { background: #ddd; }
        a { color: #0366d6; text-decoration: none; }
    </style>
</head>
<body>

    <h1>🦀 First API Axum</h1>

    <p>A simple <strong>CRUD API</strong> in <strong>Rust</strong> using <strong>Axum</strong> to manage users.</p>

    <p>
        <a href="https://www.rust-lang.org/">![Rust](https://img.shields.io/badge/Rust-1.72-blue?logo=rust)</a>
        <a href="https://docs.rs/axum/latest/axum/">![Axum](https://img.shields.io/badge/Axum-0.7.0-lightgrey?logo=rust)</a>
    </p>

    <h2>📖 Overview</h2>
    <p>This project is a <strong>basic user CRUD API</strong> that allows you to:</p>
    <ul>
        <li>List all users</li>
        <li>Get a user by ID</li>
        <li>Create a new user</li>
        <li>Update an existing user</li>
        <li>Delete a user</li>
    </ul>
    <p>Users are stored in memory using a <code>HashMap</code> wrapped in <code>Arc&lt;Mutex&lt;_&gt;&gt;</code> for thread-safe access.</p>

    <h2>🛠 Technologies</h2>
    <ul>
        <li><strong>Rust</strong> – programming language</li>
        <li><strong>Axum</strong> – web framework</li>
        <li><strong>Serde</strong> – serialization & deserialization</li>
        <li><strong>Tokio</strong> – async runtime</li>
        <li><strong>Tracing</strong> – logging</li>
    </ul>

    <h2>🚀 Getting Started</h2>
    <ol>
        <li>Clone the repository:
            <pre><code>git clone https://github.com/your_username/first_api_axum.git
cd first_api_axum</code></pre>
        </li>
        <li>Build and run:
            <pre><code>cargo run</code></pre>
        </li>
    </ol>
    <p>The API will run at: <code>http://127.0.0.1:3000</code></p>

    <h2>🌐 Endpoints</h2>
    <table>
        <tr>
            <th>Method</th>
            <th>Route</th>
            <th>Description</th>
        </tr>
        <tr><td>GET</td><td>/usuarios</td><td>List all users</td></tr>
        <tr><td>GET</td><td>/usuarios/:id</td><td>Get a user by ID</td></tr>
        <tr><td>POST</td><td>/usuarios</td><td>Create a new user</td></tr>
        <tr><td>PUT</td><td>/usuarios/:id</td><td>Update an existing user</td></tr>
        <tr><td>DELETE</td><td>/usuarios/:id</td><td>Delete a user</td></tr>
    </table>

    <h3>Example JSON (for create/update)</h3>
    <pre><code>{
  "id": 2,
  "nombre": "Alice",
  "edad": 25
}</code></pre>

    <h2>🧩 Code Structure</h2>
    <ul>
        <li><code>AppState</code> – global shared state</li>
        <li><code>Usuario</code> – user data model</li>
        <li>Handlers – route logic (<code>listar_usuarios</code>, <code>obtener_usuario</code>, etc.)</li>
        <li><code>main</code> – server setup and routing</li>
    </ul>

    <h2>⚡ Key Features</h2>
    <ul>
        <li>Thread-safe state with <code>Arc&lt;Mutex&lt;HashMap&gt;&gt;</code></li>
        <li>Basic validation (non-empty name, age &gt; 0)</li>
        <li>Proper HTTP responses (<code>200 OK</code>, <code>201 Created</code>, <code>404 Not Found</code>, etc.)</li>
        <li>Logging with <code>tracing_subscriber</code></li>
    </ul>

    <h2>📌 Notes</h2>
    <p>This is an <strong>educational project</strong>. It does <strong>not persist data</strong>. For production projects, consider using <strong>SQL</strong> or <strong>MongoDB</strong> with proper migration and persistence.</p>

    <h2>💻 Example Usage with <code>curl</code></h2>

    <p><strong>List users:</strong></p>
    <pre><code>curl http://127.0.0.1:3000/usuarios</code></pre>

    <p><strong>Create user:</strong></p>
    <pre><code>curl -X POST http://127.0.0.1:3000/usuarios \
  -H "Content-Type: application/json" \
  -d '{"id":2,"nombre":"Alice","edad":25}'</code></pre>

    <p><strong>Update user:</strong></p>
    <pre><code>curl -X PUT http://127.0.0.1:3000/usuarios/2 \
  -H "Content-Type: application/json" \
  -d '{"id":2,"nombre":"Alice","edad":26}'</code></pre>

    <p><strong>Delete user:</strong></p>
    <pre><code>curl -X DELETE http://127.0.0.1:3000/usuarios/2</code></pre>

</body>
</html>
