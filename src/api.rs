use crate::tmux::Tmux;
use serde_json::{Value, json};
use warp::{
    http::StatusCode as SC,
    reply::{json as wjson, with_status as wstatus},
};
pub struct Api;

impl Api {
    pub async fn kill_pane_id(id: &str) -> impl warp::Reply + use<> {
        if !id.starts_with("%") || id[1..].parse::<u16>().is_err() {
            return wstatus(
                wjson(&json!({"success": false, "error": "expected %<number>"})),
                SC::BAD_REQUEST,
            );
        }

        match Tmux::kill_pane_id(id) {
            Ok(_) => wstatus(
                wjson(&json!({"success": true, "message": format!("pane id {id} killed")})),
                SC::OK,
            ),
            Err(e) => wstatus(
                wjson(&json!({"success": false, "error": e.to_string()})),
                SC::INTERNAL_SERVER_ERROR,
            ),
        }
    }

    pub async fn select_pane_id(id: &str) -> impl warp::Reply + use<> {
        if !id.starts_with("%") || id[1..].parse::<u16>().is_err() {
            return wstatus(
                wjson(&json!({"success": false, "error": "expected %<number>"})),
                SC::BAD_REQUEST,
            );
        }
        match Tmux::select_pane_id(id) {
            Ok(_) => wstatus(
                wjson(&json!({"success": true, "message": format!("pane id {id} selected")})),
                SC::OK,
            ),
            Err(e) => wstatus(
                wjson(&json!({"success": false, "error": e.to_string()})),
                SC::INTERNAL_SERVER_ERROR,
            ),
        }
    }

    pub async fn capture_pane_id(id: &str) -> impl warp::Reply + use<> {
        if !id.starts_with("%") || id[1..].parse::<u16>().is_err() {
            return wstatus(
                wjson(&json!({"success": false, "error": "expected %<number>"})),
                SC::BAD_REQUEST,
            );
        }
        match Tmux::capture_pane_id(id) {
            Ok(out) => wstatus(wjson(&json!({"success": true, "output": out})), SC::OK),
            Err(e) => wstatus(
                wjson(&json!({"success": false, "error": e.to_string()})),
                SC::INTERNAL_SERVER_ERROR,
            ),
        }
    }

    pub async fn read_pane(target: &str) -> impl warp::Reply + use<> {
        if !target.contains(".") || !target.contains(":") {
            return wstatus(
                wjson(&json!({"success": false, "error": "format = session:win.pane"})),
                SC::BAD_REQUEST,
            );
        }
        // session:win.pane
        let parts = target.split(":").collect::<Vec<&str>>();
        let session = parts[0];
        let parts2 = parts[1].split(".").collect::<Vec<&str>>();

        let window = match parts2[0].parse::<u16>() {
            Ok(w) => w,
            Err(_) => {
                return wstatus(
                    wjson(&json!({"success": false, "error": "invalid window id"})),
                    SC::BAD_REQUEST,
                );
            }
        };

        let pane = match parts2[1].parse::<u16>() {
            Ok(p) => p,
            Err(_) => {
                return wstatus(
                    wjson(&json!({"success": false, "error": "invalid pane id"})),
                    SC::BAD_REQUEST,
                );
            }
        };

        match Tmux::capture_pane(session, window, pane) {
            Ok(dat) => wstatus(wjson(&json!({"success": true, "output": dat})), SC::OK),
            Err(e) => wstatus(
                wjson(&json!({"success": false, "error": e.to_string() })),
                SC::INTERNAL_SERVER_ERROR,
            ),
        }
    }

    pub async fn kill_pane(target: &str) -> impl warp::Reply + use<> {
        if !target.contains(".") || !target.contains(":") {
            return wstatus(
                wjson(&json!({"success": false, "error": "format = session:win.pane"})),
                SC::BAD_REQUEST,
            );
        }
        // session:win.pane
        let parts = target.split(":").collect::<Vec<&str>>();
        let session = parts[0];
        let parts2 = parts[1].split(".").collect::<Vec<&str>>();

        let window = match parts2[0].parse::<u16>() {
            Ok(w) => w,
            Err(_) => {
                return wstatus(
                    wjson(&json!({"success": false, "error": "invalid window id"})),
                    SC::BAD_REQUEST,
                );
            }
        };

        let pane = match parts2[1].parse::<u16>() {
            Ok(p) => p,
            Err(_) => {
                return wstatus(
                    wjson(&json!({"success": false, "error": "invalid pane id"})),
                    SC::BAD_REQUEST,
                );
            }
        };

        match Tmux::kill_pane(session, window, pane) {
            Ok(_) => wstatus(
                wjson(&json!({"success": true, "message": format!("pane {target} killed!")})),
                SC::OK,
            ),
            Err(e) => wstatus(
                wjson(&json!({"success": false, "error": e.to_string() })),
                SC::INTERNAL_SERVER_ERROR,
            ),
        }
    }

    pub async fn select_pane(target: &str) -> impl warp::Reply + use<> {
        if !target.contains(".") || !target.contains(":") {
            return wstatus(
                wjson(&json!({"success": false, "error": "format = session:win.pane"})),
                SC::BAD_REQUEST,
            );
        }
        // session:win.pane
        let parts = target.split(":").collect::<Vec<&str>>();
        let session = parts[0];
        let parts2 = parts[1].split(".").collect::<Vec<&str>>();

        let window = match parts2[0].parse::<u16>() {
            Ok(w) => w,
            Err(_) => {
                return wstatus(
                    wjson(&json!({"success": false, "error": "invalid window id"})),
                    SC::BAD_REQUEST,
                );
            }
        };

        let pane = match parts2[1].parse::<u16>() {
            Ok(p) => p,
            Err(_) => {
                return wstatus(
                    wjson(&json!({"success": false, "error": "invalid pane id"})),
                    SC::BAD_REQUEST,
                );
            }
        };

        match Tmux::select_pane(session, window, pane) {
            Ok(_) => wstatus(
                wjson(&json!({"success": true, "message": format!("pane {target} selected!")})),
                SC::OK,
            ),
            Err(e) => wstatus(
                wjson(&json!({"success": false, "error": e.to_string()})),
                SC::INTERNAL_SERVER_ERROR,
            ),
        }
    }

    pub async fn list_panes(target: &str) -> impl warp::Reply + use<> {
        let parts = target.split(":").collect::<Vec<&str>>();

        if parts.len() != 2 {
            return wstatus(
                wjson(&json!({
                    "success": false,
                    "error": "incorrect use, expected session_name:window_id"
                })),
                SC::BAD_REQUEST,
            );
        }

        let session_name = parts[0];

        let win_idx = match parts[1].parse::<u16>() {
            Ok(v) => v,
            Err(_) => {
                return wstatus(
                    wjson(&json!({"success": false, "error": "window id should be a number"})),
                    SC::BAD_REQUEST,
                );
            }
        };

        match Tmux::list_panes(session_name, win_idx) {
            Ok(res) => {
                let panes = res.split("\n").collect::<Vec<&str>>();
                let result = panes
                    .iter()
                    .map(|f| {
                        let data = f.split("|").collect::<Vec<&str>>();
                        let is_active = data[2].parse::<u16>().unwrap_or(0) == 1;

                        let pane_command = match data[6].parse::<u64>() {
                            Ok(pid) if pid > 0 => {
                                Tmux::get_cmd(pid).unwrap_or_else(|_| "unknown".into())
                            }
                            _ => "unknown".into(),
                        };

                        json!({
                            "index": data[0],
                            "id": data[1],
                            "active":is_active,
                            "width": data[3],
                            "height": data[4],
                            "last_program": data[5],
                            "pid": data[6],
                            "command": if pane_command.is_empty() { "empty".into() } else { pane_command }
                        })
                    })
                    .collect::<Vec<Value>>();
                wstatus(wjson(&json!({"success": true, "panes": result})), SC::OK)
            }
            Err(e) => wstatus(
                wjson(&json!({"success": false, "error": e.to_string()})),
                SC::INTERNAL_SERVER_ERROR,
            ),
        }
    }

    pub async fn split_window(target: &str, orientation: &str) -> impl warp::Reply + use<> {
        // target = <session_name>:<window_id>
        let parts = target.split(":").collect::<Vec<&str>>();

        if parts.len() != 2 {
            return wstatus(
                wjson(&json!({
                    "success": false,
                    "error": "incorrect use, expected session_name:window_id"
                })),
                SC::BAD_REQUEST,
            );
        }

        let tsession = parts[0];

        let win_idx = match parts[1].parse::<u16>() {
            Ok(v) => v,
            Err(_) => {
                return wstatus(
                    wjson(&json!({"success": false, "error": "window id should be a number"})),
                    SC::BAD_REQUEST,
                );
            }
        };

        let or = if orientation == "vertical" || orientation == "vert" || orientation == "v" {
            "-v"
        } else if orientation == "horizontal" || orientation == "hoz" || orientation == "h" {
            "-h"
        } else {
            return wstatus(
                wjson(&json!({"success": false, "error": "invalid orientation"})),
                SC::BAD_REQUEST,
            );
        };

        match Tmux::split_window(tsession, win_idx, or) {
            Ok(_) => wstatus(
                wjson(
                    &json!({"success": true, "message": format!("window {target} split {orientation}!")}),
                ),
                SC::OK,
            ),
            Err(e) => wstatus(
                wjson(&json!({"success": false, "error": e.to_string()})),
                SC::INTERNAL_SERVER_ERROR,
            ),
        }
    }

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
                    &json!({"success":false, "error": "Invalid Usage, expected session_name:window_id"}),
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
        match Tmux::kill_window(session_name, idx) {
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
