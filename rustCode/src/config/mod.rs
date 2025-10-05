use dotenvy::from_path;
use std::{env, path::Path};

pub fn load_env() {
    let env_path = Path::new("../goCode/.env");
    from_path(env_path).expect("Nije moguce ucitati .env fajl sa date putanje");
}

pub fn get_rpc_url() -> String {
    env::var("RPC_URL").expect("RPC_URL nije definisan")
}
