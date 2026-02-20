use crate::tmux::Tmux;
use serde_json::{Value, json};
use warp::{
    http::StatusCode as SC,
    reply::{json as wjson, with_status as wstatus},
};
pub struct Api;

impl Api {
    pub async fn new_window(session_name: &str, window_name: &str) -> impl warp::Reply + use<> {
        match Tmux::new_window(session_name, window_name) {
            Ok(_) => wstatus(
                wjson(
                    &json!({"success": true, "message": format!("window {session_name}:{window_name} created!")}),
                ),
                SC::OK,
            ),
            Err(e) => wstatus(
                wjson(&json!({"success": false, "error": e.to_string()})),
                SC::INTERNAL_SERVER_ERROR,
            ),
        }
    }

    pub async fn kill_window(target: String) -> impl warp::Reply + use<> {
        let parts = &target.split(":").collect::<Vec<&str>>();

        if parts.len() != 2 {
            return wstatus(
                wjson(
                    &json!({"success":false, "message": "Invalid Usage, expected session_name:window_id"}),
                ),
                SC::BAD_REQUEST,
            );
        }
        let session_name = parts[0];
        let idx = match parts[1].parse::<u16>() {
            Ok(i) => i,
            Err(_) => {
                return wstatus(
                    wjson(&json!({"success": false, "message": "window id possibly not a number"})),
                    SC::BAD_REQUEST,
                );
            }
        };
        match Tmux::kill_window(&session_name, idx) {
            Ok(_) => wstatus(
                wjson(
                    &json!({"success": true, "message":format!("window {session_name}:{idx} killed!")}),
                ),
                SC::OK,
            ),
            Err(e) => wstatus(
                wjson(&json!({"success": false, "error":e.to_string()})),
                SC::INTERNAL_SERVER_ERROR,
            ),
        }
    }

    pub async fn list_windows(session_name: &str) -> impl warp::Reply + use<> {
        match Tmux::list_windows(session_name) {
            Ok(res) => {
                let windows = res.split("\n").collect::<Vec<&str>>();
                let result = windows
                    .iter()
                    .map(|s| {
                        let parts = s.split("|").collect::<Vec<&str>>();
                        json!({
                            "name": parts[1],
                            "index": parts[0],
                            "active": parts[2]
                        })
                    })
                    .collect::<Vec<Value>>();

                wstatus(wjson(&json!({"success": true, "windows": result})), SC::OK)
            }
            Err(e) => wstatus(
                wjson(&json!({"success": false, "error":e.to_string()})),
                SC::INTERNAL_SERVER_ERROR,
            ),
        }
    }

    pub async fn new_session(name: &str) -> impl warp::Reply + use<> {
        match Tmux::new_session(name) {
            Ok(_) => wstatus(
                wjson(&json!({"success": true, "message":format!("session {name} created!")})),
                SC::OK,
            ),
            Err(e) => wstatus(
                wjson(&json!({"success": false, "error":e.to_string()})),
                SC::INTERNAL_SERVER_ERROR,
            ),
        }
    }

    pub async fn kill_session(name: &str) -> impl warp::Reply + use<> {
        match Tmux::kill_session(name) {
            Ok(_) => {
                // tmux won't reply to this command,
                // this becomes successful
                wstatus(
                    wjson(&json!({"success": true, "message":format!("session {name} killed!")})),
                    SC::OK,
                )
            }
            Err(e) => wstatus(
                wjson(&json!({"success": false, "error":e.to_string()})),
                SC::INTERNAL_SERVER_ERROR,
            ),
        }
    }

    pub async fn list_sessions() -> impl warp::Reply {
        match Tmux::list_sessions() {
            Ok(out) => {
                let sessions = out.split("\n").collect::<Vec<&str>>();
                let out = sessions
                    .iter()
                    .map(|s| {
                        let parts = s.split("|").collect::<Vec<&str>>();
                        json!({
                            "name": parts[0],
                            "windows": parts[1],
                            "created_at": parts[2]
                        })
                    })
                    .collect::<Vec<Value>>();
                wstatus(wjson(&json!({"success": true, "sessions": &out})), SC::OK)
            }
            Err(e) => wstatus(
                wjson(&json!({"success": false, "error":e.to_string()})),
                SC::INTERNAL_SERVER_ERROR,
            ),
        }
    }
}
