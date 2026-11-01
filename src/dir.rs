use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize)]
struct SessionStart {
    session_id: String,
}

pub fn run() {
    let payload: SessionStart =
        serde_json::from_reader(std::io::stdin()).expect("failed to parse JSON from stdin");

    std::fs::create_dir_all(vaults_path().join(payload.session_id))
        .expect("failed to create session directory");
}

fn vaults_path() -> PathBuf {
    let vaults_path = std::env::var("CLAUDE_CODE_SESSION_LOCAL_VAULT_DIR")
        .expect("CLAUDE_CODE_SESSION_LOCAL_VAULT_DIR is not set");

    PathBuf::from(vaults_path)
}
