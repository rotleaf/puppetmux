pub struct ShortcutsApi;
use serde_json::json;
use warp::{
    http::StatusCode as SC,
    reply::{Json, WithStatus, json as wjson, with_status as wstatus},
};

use crate::tmux::Tmux;

const SHELLS: &[&str] = &["bash", "zsh", "sh", "fish", "dash", "csh", "tcsh", "ksh"];

// reading this shit?
// wpid = with pane id, takes in a pane id

pub fn valid_pane_id(id: &str) -> bool {
    if !id.starts_with("%") || id[1..].parse::<u64>().is_err() {
        return false;
    }
    true
}

pub struct PaneData {
    pub current_command: String,
    pub _process_pid: u32,
}

impl ShortcutsApi {
    pub fn pane_data(id: &str) -> PaneData {
        let output = Tmux::run(&[
            "display-message",
            "-t",
            id,
            "-p",
            "#{pane_current_command}|#{pane_pid}",
        ])
        .unwrap_or("unknown|0".to_string());

        let parts = output.split('|').collect::<Vec<&str>>();

        PaneData {
            current_command: parts[0].into(),
            _process_pid: parts[1].parse::<u32>().unwrap_or(0),
        }
    }

    fn check_pane(id: &str) -> Result<PaneData, WithStatus<Json>> {
        let pane_data = Self::pane_data(id);
        if !valid_pane_id(id) {
            return Err(wstatus(
                wjson(&json!({"success": false, "error": "invalid pane id, required %<number>"})),
                SC::BAD_REQUEST,
            ));
        }
        if !SHELLS.contains(&pane_data.current_command.as_str().trim()) {
            return Err(wstatus(
                wjson(&json!({"success": true, "message": format!("pane {id} is not idle")})),
                SC::OK,
            ));
        }

        Ok(pane_data)
    }

    pub async fn ctrl_c(id: &str) -> impl warp::Reply + use<> {
        match Tmux::send_keys(id, &["C-c"]) {
            Ok(_) => wstatus(
                wjson(&json!({"success": true, "message": format!("Ctrl+C sent to window {id}")})),
                SC::OK,
            ),
            Err(e) => wstatus(
                wjson(&json!({"success": false, "error": e.to_string()})),
                SC::INTERNAL_SERVER_ERROR,
            ),
        }
    }

    // check what was the last command
    pub async fn check_last_command_wpid(id: &str) -> impl warp::Reply + use<> {
        let _pane_data = match Self::check_pane(id) {
            Ok(pd) => pd,
            Err(rep) => return rep,
        };

        // send arrow up, capture pane
        if Tmux::send_keys(id, &["Up"]).is_ok() {
            let pane_content = match Tmux::capture_pane_id(id) {
                Ok(cont) => cont,
                Err(e) => {
                    return wstatus(
                        wjson(&json!({"success": false, "error": e.to_string()})),
                        SC::INTERNAL_SERVER_ERROR,
                    );
                }
            };

            let last_line = pane_content.lines().last().unwrap_or("").trim();

            let command = last_line
                .trim()
                .trim_start_matches(|c: char| !c.is_alphanumeric() && c != '/' && c != '.')
                .trim();

            wstatus(wjson(&json!({"success":true, "command": command})), SC::OK)
        } else {
            wstatus(
                wjson(&json!({"success": false, "error": "failed to send up key"})),
                SC::INTERNAL_SERVER_ERROR,
            )
        }
    }

    // run the most last command, arrow up + enter
    pub async fn last_command_wpid(id: &str) -> impl warp::Reply + use<> {
        let _pane_data = match Self::check_pane(id) {
            Ok(pd) => pd,
            Err(rep) => return rep,
        };

        // let cmd = Tmux::get_cmd(pane_data.process_pid.into()).unwrap_or_else(|_| "unknown".into());
        // this will be pid for zsh if program stopped
        match Tmux::send_keys(id, &["Up", "Enter"]) {
            Ok(_) => wstatus(
                wjson(&json!({"success": true, "message": format!("last command rerun!")})),
                SC::OK,
            ),
            Err(e) => wstatus(
                wjson(&json!({"success": false, "error": e.to_string()})),
                SC::INTERNAL_SERVER_ERROR,
            ),
        }
    }
}
