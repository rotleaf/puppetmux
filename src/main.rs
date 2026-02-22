mod api;
mod shortcuts_api;
mod tmux;

use crate::api::Api;
use colored::Colorize;
use rand::RngExt;
use serde_json::json;
use shortcuts_api::ShortcutsApi as SApi;
use warp::Filter;

use warp::{
    http::StatusCode as SC,
    reply::{json as wjson, with_status as wstatus},
};
pub type Ret<T> = Result<T, Box<dyn std::error::Error>>;

pub fn randstr() -> String {
    rand::rng()
        .sample_iter(rand::distr::Alphanumeric)
        .take(8)
        .map(char::from)
        .collect()
}

#[derive(Debug)]
struct Unauthorised;
impl warp::reject::Reject for Unauthorised {}

#[derive(Debug)]
struct NoApiKey;
impl warp::reject::Reject for NoApiKey {}

fn auth() -> impl Filter<Extract = (), Error = warp::Rejection> + Clone {
    warp::header::optional::<String>("api-key")
        .and_then(|key: Option<String>| async move {
            let mykey = match std::env::var("API_KEY") {
                Ok(k) => k,
                Err(_) => return Ok(()),
            };
            match key {
                Some(k) if k == mykey => Ok(()),
                Some(_) => Err(warp::reject::custom(Unauthorised)),
                None => Err(warp::reject::custom(Unauthorised)),
            }
        })
        .untuple_one()
}

async fn handle_rejection(err: warp::Rejection) -> Result<impl warp::Reply, warp::Rejection> {
    if err.find::<Unauthorised>().is_some() {
        return Ok(wstatus(
            wjson(&json!({"success": false, "error": "Invalid api key"})),
            SC::UNAUTHORIZED,
        ));
    }

    if err.find::<NoApiKey>().is_some() {
        return Ok(wstatus(
            wjson(&json!({"success": false, "error": "API_KEY env var not set"})),
            SC::INTERNAL_SERVER_ERROR,
        ));
    }

    if err.find::<warp::reject::MissingHeader>().is_some() {
        return Ok(wstatus(
            wjson(&json!({"success": false, "error": "missing api-key header"})),
            SC::UNAUTHORIZED,
        ));
    }
    Err(err)
}

#[tokio::main]
async fn main() {
    // get sessions
    let list_sessions = warp::path!("session" / "list")
        .and(warp::get())
        .and_then(|| async { Ok::<_, warp::Rejection>(Api::list_sessions().await) });

    // 404
    let not_found = warp::any().and_then(|| async {
        Ok::<_, warp::Rejection>(wstatus(
            wjson(&json!({"success": false, "error": "not found"})),
            SC::NOT_FOUND,
        ))
    });

    let kill_session = warp::path!("session" / "kill" / String)
        .and(warp::get())
        .and_then(|name: String| async move {
            Ok::<_, warp::Rejection>(Api::kill_session(&name).await)
        });

    let new_session = warp::path!("session" / "new" / String)
        .and(warp::get())
        .and_then(
            |name: String| async move { Ok::<_, warp::Rejection>(Api::new_session(&name).await) },
        );

    let new_session_ = warp::path!("session" / "new")
        .and(warp::get())
        .and_then(|| async {
            let name: String = randstr();
            Ok::<_, warp::Rejection>(Api::new_session(&name).await)
        });

    let list_windows = warp::path!("window" / "list" / String)
        .and(warp::get())
        .and_then(|sname: String| async move {
            Ok::<_, warp::Rejection>(Api::list_windows(&sname).await)
        });

    let kill_window = warp::path!("window" / "kill" / String)
        .and(warp::get())
        .and_then(|target: String| async move {
            Ok::<_, warp::Rejection>(Api::kill_window(target).await)
        });

    // new window, random name
    let new_window_ = warp::path!("window" / "new" / String)
        .and(warp::get())
        .and_then(|sess: String| async move {
            let winname: String = randstr();
            Ok::<_, warp::Rejection>(Api::new_window(&sess, &winname).await)
        });

    // new window, defined name
    let new_window = warp::path!("window" / "new" / String / String)
        .and(warp::get())
        .and_then(|sess: String, win: String| async move {
            Ok::<_, warp::Rejection>(Api::new_window(&sess, &win).await)
        });

    // split  awindow, /window/split/<session>:<winid>/orientation
    let split_window = warp::path!("window" / "split" / String / String)
        .and(warp::get())
        .and_then(|targets: String, orientation: String| async move {
            Ok::<_, warp::Rejection>(Api::split_window(&targets, &orientation).await)
        });

    let list_panes = warp::path!("pane" / "list" / String)
        .and(warp::get())
        .and_then(|target: String| async move {
            Ok::<_, warp::Rejection>(Api::list_panes(&target).await)
        });

    let select_pane = warp::path!("pane" / "select" / String)
        .and(warp::get())
        .and_then(|target: String| async move {
            Ok::<_, warp::Rejection>(Api::select_pane(&target).await)
        });

    let kill_pane = warp::path!("pane" / "kill" / String)
        .and(warp::get())
        .and_then(
            |targ: String| async move { Ok::<_, warp::Rejection>(Api::kill_pane(&targ).await) },
        );

    let read_pane = warp::path!("pane" / "read" / String)
        .and(warp::get())
        .and_then(
            |targ: String| async move { Ok::<_, warp::Rejection>(Api::read_pane(&targ).await) },
        );

    let read_pane_id = warp::path!("pane" / String / "read")
        .and(warp::get())
        .and_then(
            |id: String| async move { Ok::<_, warp::Rejection>(Api::capture_pane_id(&id).await) },
        );

    let kill_pane_id = warp::path!("pane" / String / "kill")
        .and(warp::get())
        .and_then(
            |id: String| async move { Ok::<_, warp::Rejection>(Api::kill_pane_id(&id).await) },
        );

    // pane id %<number>
    let select_pane_id = warp::path!("pane" / String / "select")
        .and(warp::get())
        .and_then(
            |id: String| async move { Ok::<_, warp::Rejection>(Api::select_pane_id(&id).await) },
        );

    // run last cmd
    let last_cmd_run = warp::path!("pane" / String / "last-cmd" / "run")
        .and(warp::get())
        .and_then(|pane_id: String| async move {
            Ok::<_, warp::Rejection>(SApi::last_command_wpid(&pane_id).await)
        });

    // check last cmd
    let last_cmd_check = warp::path!("pane" / String / "last-cmd")
        .and(warp::get())
        .and_then(|pid: String| async move {
            Ok::<_, warp::Rejection>(SApi::check_last_command_wpid(&pid).await)
        });
    // send ctrl+c
    let ctrl_c_wpid = warp::path!("pane" / String / "ctrl-c")
        .and(warp::get())
        .and_then(|pid: String| async move { Ok::<_, warp::Rejection>(SApi::ctrl_c(&pid).await) });

    let logger = warp::log::custom(|info| {
        let status = info.status();
        let cst = match status.as_u16() {
            200..=299 => format!("{}", status.as_str().green().bold()),
            300..=399 => format!("{}", status.as_str().yellow().bold()),
            400..=499 => format!("{}", status.as_str().red().bold()),
            500..=599 => format!("{}", status.as_str().magenta().bold()),
            _ => format!("{}", status),
        };
        println!(" 🪵 {} {} {}", cst, info.method(), info.path());
    });

    let routes = auth()
        .and(
            list_sessions
                .or(new_session_) // random session name
                .or(kill_window)
                .or(new_session) // named session
                .or(kill_session)
                .or(list_windows)
                .or(new_window_) // random window name
                .or(new_window) // named window
                .or(split_window)
                .or(list_panes)
                .or(select_pane)
                .or(select_pane_id)
                .or(kill_pane)
                .or(kill_pane_id)
                .or(read_pane)
                .or(read_pane_id)
                .or(last_cmd_run)
                .or(last_cmd_check)
                .or(ctrl_c_wpid)
                .or(not_found),
        )
        .recover(handle_rejection)
        .with(logger);

    println!(" » {} running on port 3030", "PuppetMux".bold().blue());
    let auth_enabled = if std::env::var("API_KEY").is_ok() {
        format!("{}", "On".green().bold())
    } else {
        format!(
            "{} ({})",
            "Off".red().bold(),
            "set API_KEY env var to enable auth".bright_black()
        )
    };
    println!(" » Auth: {auth_enabled}");
    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}
