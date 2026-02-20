use crate::tmux::Tmux;
use serde_json::{Value, json};

pub struct Api;

impl Api {
    pub async fn list_windows(session_name: &str) -> impl warp::Reply + use<> {
        match Tmux::list_windows(session_name) {
            Ok(res) => {
                let windows = res.split("\n").collect::<Vec<&str>>();
                let result = windows
                    .into_iter()
                    .map(|s| {
                        let parts = s.split("|").collect::<Vec<&str>>();
                        json!({
                            "name": parts[1],
                            "index": parts[0],
                            "active": parts[2]
                        })
                    })
                    .collect::<Vec<Value>>();
                warp::reply::with_status(warp::reply::json(&result), warp::http::StatusCode::OK)
            }
            Err(e) => warp::reply::with_status(
                warp::reply::json(&json!({"error":e.to_string()})),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            ),
        }
    }

    pub async fn new_session(name: &str) -> impl warp::Reply + use<> {
        match Tmux::new_session(name) {
            Ok(_) => warp::reply::with_status(
                warp::reply::json(&json!({"message":format!("session {name} created!")})),
                warp::http::StatusCode::OK,
            ),
            Err(e) => warp::reply::with_status(
                warp::reply::json(&json!({"error":e.to_string()})),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            ),
        }
    }

    pub async fn kill_session(name: &str) -> impl warp::Reply + use<> {
        match Tmux::kill_session(name) {
            Ok(_) => {
                // tmux won't reply to this command,
                // this becomes successful
                warp::reply::with_status(
                    warp::reply::json(&json!({"message":format!("session {name} killed!")})),
                    warp::http::StatusCode::OK,
                )
            }
            Err(e) => warp::reply::with_status(
                warp::reply::json(&json!({"error":e.to_string()})),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            ),
        }
    }

    pub async fn list_sessions() -> impl warp::Reply {
        match Tmux::list_sessions() {
            Ok(out) => {
                let sessions = out.split("\n").collect::<Vec<&str>>();
                let out = sessions
                    .into_iter()
                    .map(|s| {
                        let parts = s.split("|").collect::<Vec<&str>>();
                        json!({
                            "name": parts[0],
                            "windows": parts[1],
                            "created_at": parts[2]
                        })
                    })
                    .collect::<Vec<Value>>();
                warp::reply::with_status(warp::reply::json(&out), warp::http::StatusCode::OK)
            }
            Err(e) => warp::reply::with_status(
                warp::reply::json(&json!({"error":e.to_string()})),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            ),
        }
    }
}
