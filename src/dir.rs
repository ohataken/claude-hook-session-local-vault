use serde::Deserialize;

#[derive(Deserialize)]
struct SessionStart {
    session_id: String,
}

pub fn run() {
}
