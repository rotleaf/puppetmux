mod api;
mod tmux;

use crate::api::Api;
use rand::RngExt;
use warp::Filter;

pub type Ret<T> = Result<T, Box<dyn std::error::Error>>;

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

    let new_session_named = warp::path!("session" / "new" / String)
        .and(warp::get())
        .and_then(
            |name: String| async move { Ok::<_, warp::Rejection>(Api::new_session(&name).await) },
        );

    let new_session_ = warp::path!("session" / "new")
        .and(warp::get())
        .and_then(|| async {
            let name: String = rand::rng()
                .sample_iter(rand::distr::Alphanumeric)
                .take(8)
                .map(char::from)
                .collect();
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

    let routes = list_sessions
        .or(new_session_)
        .or(kill_window)
        .or(new_session_named)
        .or(kill_session)
        .or(list_windows)
        .or(not_found);

    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}
