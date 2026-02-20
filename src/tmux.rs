use crate::Ret;
use std::process::Command;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TmuxErr {
    #[error("[tmux] {0}")]
    TmuxE(String),
    #[error("[io] {0}")]
    Io(#[from] std::io::Error),
}

pub struct Tmux;

impl Tmux {
    pub fn run(args: &[&str]) -> Result<String, TmuxErr> {
        let out = Command::new("tmux").args(args).output()?;
        if out.status.success() {
            Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
        } else {
            Err(TmuxErr::TmuxE(
                String::from_utf8_lossy(&out.stderr).trim().to_string(),
            ))
        }
    }

    pub fn list_sessions() -> Ret<String> {
        Ok(Self::run(&[
            "ls",
            "-F",
            "#{session_name}|#{session_windows}|#{session_created}",
        ])?)
    }

    pub fn kill_session(name: &str) -> Ret<String> {
        Ok(Self::run(&["kill-session", "-t", name])?)
    }

    pub fn new_session(session_name: &str) -> Ret<String> {
        Ok(Self::run(&["new-session", "-d", "-s", session_name])?)
    }

    pub fn list_windows(session_name: &str) -> Ret<String> {
        Ok(Self::run(&[
            "list-windows",
            "-t",
            session_name,
            "-F",
            "#{window_index}|#{window_name}|#{window_active}",
        ])?)
    }

    pub fn kill_window(session_name: &str, win_idx: u16) -> Ret<String> {
        Ok(Self::run(&[
            "kill-window",
            "-t",
            &format!("{session_name}:{win_idx}"),
        ])?)
    }
}
