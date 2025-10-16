use axum::{
    routing::get,
    Router,
};
use std::{collections::HashMap, net::SocketAddr, sync::{Arc, Mutex}};

mod handles;
use handles::{listar_usuarios, crear_usuario, obtener_usuario, actualizar_usuario, eliminar_usuario};

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Usuario {
    pub id: u32,
    pub nombre: String,
    pub edad: u8,
}

#[derive(Clone)]
pub struct AppState {
    pub usuarios: Arc<Mutex<HashMap<u32, Usuario>>>,
}

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

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
