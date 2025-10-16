use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use crate::{Usuario, AppState};


// ================== HANDLERS ==================

// GET /usuarios
pub async fn listar_usuarios(State(state): State<AppState>) -> Json<Vec<Usuario>> {
    let usuarios = state.usuarios.lock().unwrap();
    tracing::info!("Listando todos los usuarios");
    Json(usuarios.values().cloned().collect())
}

// GET /usuarios/:id
pub async fn obtener_usuario(
    Path(id): Path<u32>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let usuarios = state.usuarios.lock().unwrap();
    
    match usuarios.get(&id) {
        Some(usuario) => {
            tracing::info!(id = usuario.id, nombre = %usuario.nombre, "Usuario encontrado");
            (StatusCode::OK, Json(usuario.clone())).into_response()
        }
        None => {
            tracing::warn!(id = id, "Intento de acceder a usuario no existente");
            (StatusCode::NOT_FOUND, Json("Usuario no encontrado")).into_response()
        }
    }
}

// POST /usuarios
pub async fn crear_usuario(
    State(state): State<AppState>,
    Json(usuario): Json<Usuario>,
) -> impl IntoResponse {
    if usuario.nombre.is_empty() || usuario.edad == 0 {
        tracing::warn!("Intento de crear usuario con datos incompletos");
        return (StatusCode::BAD_REQUEST, Json("Nombre y edad son obligatorios")).into_response();
    }

    let mut usuarios = state.usuarios.lock().unwrap();
    
    if usuarios.contains_key(&usuario.id) {
        tracing::warn!(id = usuario.id, "Intento de crear usuario ya existente");
        return (StatusCode::CONFLICT, Json("Usuario ya existe")).into_response();
    }

    usuarios.insert(usuario.id, usuario.clone());
    tracing::info!(id = usuario.id, nombre = %usuario.nombre, "Usuario creado");
    (StatusCode::CREATED, Json(usuario)).into_response()
}

// PUT /usuarios/:id
pub async fn actualizar_usuario(
    Path(id): Path<u32>,
    State(state): State<AppState>,
    Json(mut datos): Json<Usuario>,
) -> impl IntoResponse {
    let mut usuarios = state.usuarios.lock().unwrap();
    datos.id = id;

    if let Some(usuario) = usuarios.get_mut(&id) {
        *usuario = datos.clone();
        tracing::info!(id = usuario.id, nombre = %usuario.nombre, "Usuario actualizado");
        (StatusCode::OK, Json(datos)).into_response()
    } else {
        tracing::warn!(id = id, "Intento de actualizar usuario no existente");
        (StatusCode::NOT_FOUND, Json("Usuario no encontrado")).into_response()
    }
}

// DELETE /usuarios/:id
pub async fn eliminar_usuario(
    Path(id): Path<u32>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let mut usuarios = state.usuarios.lock().unwrap();
    match usuarios.remove(&id) {
        Some(_) => {
            tracing::info!(id = id, "Usuario eliminado");
            (StatusCode::OK, Json("Usuario eliminado")).into_response()
        }
        None => {
            tracing::warn!(id = id, "Intento de eliminar usuario no existente");
            (StatusCode::NOT_FOUND, Json("Usuario no encontrado")).into_response()
        }
    }
}
