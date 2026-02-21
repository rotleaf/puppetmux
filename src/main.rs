mod api;
mod tmux;

use crate::api::Api;
use colored::Colorize;
use rand::RngExt;
use warp::Filter;

pub type Ret<T> = Result<T, Box<dyn std::error::Error>>;

pub fn randstr() -> String {
    rand::rng()
        .sample_iter(rand::distr::Alphanumeric)
        .take(8)
        .map(char::from)
        .collect()
}

#[tokio::main]
async fn main() {
    // get sessions
    let list_sessions = warp::path!("session" / "list")
        .and(warp::get())
        .and_then(|| async { Ok::<_, warp::Rejection>(Api::list_sessions().await) });

    // 404
    let not_found = warp::any().map(|| {
        warp::reply::with_status(
            warp::reply::json(&serde_json::json!({ "error": "not found" })),
            warp::http::StatusCode::NOT_FOUND,
        )
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

    let select_pane_id = warp::path!("pane" / String / "select")
        .and(warp::get())
        .and_then(
            |id: String| async move { Ok::<_, warp::Rejection>(Api::select_pane_id(&id).await) },
        );
    let logger = warp::log::custom(|info| {
        let status = info.status();
        let cst = match status.as_u16() {
            200..=299 => format!("{}", status.as_str().green().bold()),
            300..=399 => format!("{}", status.as_str().yellow().bold()),
            400..=499 => format!("{}", status.as_str().red().bold()),
            500..=599 => format!("{}", status.as_str().magenta().bold()),
            _ => format!("{}", status),
        };
        println!("{} {} {}", cst, info.method(), info.path());
    });

    let routes = list_sessions
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
        .or(not_found)
        .with(logger);

    println!("{} running on port 3030", "PuppetMux".bold().blue());
    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}
