use serde::Deserialize;

#[derive(Deserialize)]
struct SessionStart {
    session_id: String,
}

pub fn run() {
    let payload: SessionStart =
        serde_json::from_reader(std::io::stdin()).expect("failed to parse JSON from stdin");
}
