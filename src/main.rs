use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tracing_subscriber;


#[derive(Clone, Serialize, Deserialize)]
struct Usuario {
    id: u32,
    nombre: String,
    edad: u8,
}

// Estado compartido
#[derive(Clone)]
struct AppState {
    usuarios: Arc<Mutex<HashMap<u32, Usuario>>>,
}

// ================== HANDLERS ==================

// GET /usuarios
async fn listar_usuarios(State(state): State<AppState>) -> Json<Vec<Usuario>> {
    let usuarios = state.usuarios.lock().unwrap();
    Json(usuarios.values().cloned().collect())
}

// GET /usuarios/:id
async fn obtener_usuario(
    Path(id): Path<u32>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let usuarios = state.usuarios.lock().unwrap();
    match usuarios.get(&id) {
        Some(usuario) => (StatusCode::OK, Json(usuario.clone())).into_response(),
        None => (StatusCode::NOT_FOUND, Json("Usuario no encontrado")).into_response(),
    }
}

// POST /usuarios
async fn crear_usuario(
    State(state): State<AppState>,
    Json(usuario): Json<Usuario>,
) -> impl IntoResponse {
    if usuario.nombre.is_empty() || usuario.edad == 0 {
        return (StatusCode::BAD_REQUEST, Json("Nombre y edad son obligatorios")).into_response();
    }
    let mut usuarios = state.usuarios.lock().unwrap();
    
    // Verificar si el usuario ya existe
    if usuarios.contains_key(&usuario.id) {
        return (StatusCode::CONFLICT, Json("Usuario ya existe")).into_response();
    }
    
    usuarios.insert(usuario.id, usuario.clone());
    (StatusCode::CREATED, Json(usuario)).into_response()
}

// PUT /usuarios/:id
async fn actualizar_usuario(
    Path(id): Path<u32>,
    State(state): State<AppState>,
    Json(mut datos): Json<Usuario>,
) -> impl IntoResponse {
    let mut usuarios = state.usuarios.lock().unwrap();
    
    // Asegurar que el ID del path coincida con el ID del cuerpo
    datos.id = id;
    
    match usuarios.get_mut(&id) {
        Some(usuario) => {
            *usuario = datos.clone();
            (StatusCode::OK, Json(datos)).into_response()
        }
        None => (StatusCode::NOT_FOUND, Json("Usuario no encontrado")).into_response(),
    }
}

// DELETE /usuarios/:id
async fn eliminar_usuario(
    Path(id): Path<u32>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let mut usuarios = state.usuarios.lock().unwrap();
    match usuarios.remove(&id) {
        Some(_) => (StatusCode::OK, Json("Usuario eliminado")).into_response(),
        None => (StatusCode::NOT_FOUND, Json("Usuario no encontrado")).into_response(),
    }
}

// ================== MAIN ==================
#[tokio::main]
async fn main() {
    // Init logging
    tracing_subscriber::fmt::init();

    let estado = AppState {
        usuarios: Arc::new(Mutex::new(HashMap::from([(
            1,
            Usuario {
                id: 1,
                nombre: "GTS".to_string(),
                edad: 17,
            },
        )]))),
    };

    let app = Router::new()
        .route("/usuarios", get(listar_usuarios).post(crear_usuario))
        .route(
            "/usuarios/:id",
            get(obtener_usuario)
                .put(actualizar_usuario)
                .delete(eliminar_usuario),
        )
        .with_state(estado);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Servidor corriendo en http://{}", addr);

    // FORMA CORRECTA para Axum 0.7+
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}