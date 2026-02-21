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
    pub fn installed() -> bool {
        Command::new("sh")
            .args(["-c", "command -v tmux"])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    pub fn run(args: &[&str]) -> Result<String, TmuxErr> {
        if !Self::installed() {
            return Err(TmuxErr::TmuxE("tmux not installed".into()));
        }

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

    pub fn new_window(session_name: &str, window_name: &str) -> Ret<String> {
        Ok(Self::run(&[
            "new-window",
            "-d",
            "-t",
            session_name,
            "-n",
            window_name,
        ])?)
    }

    pub fn split_window(session_name: &str, win_id: u16, or: &str) -> Ret<String> {
        Ok(Self::run(&[
            "split-window",
            "-d",
            "-t",
            &format!("{session_name}:{win_id}"),
            or,
        ])?)
    }

    pub fn select_pane(session: &str, win_id: u16, pane_id: u16) -> Ret<String> {
        Ok(Self::run(&[
            "select-pane",
            "-t",
            &format!("{session}:{win_id}.{pane_id}"),
        ])?)
    }

    pub fn select_pane_id(id: &str) -> Ret<String> {
        Ok(Self::run(&["select-pane", "-t", id])?)
    }

    pub fn kill_pane(session: &str, win_id: u16, pane_id: u16) -> Ret<String> {
        Ok(Self::run(&[
            "kill-pane",
            "-t",
            &format!("{session}:{win_id}.{pane_id}"),
        ])?)
    }

    pub fn kill_pane_id(id: &str) -> Ret<String> {
        Ok(Self::run(&["kill-pane", "-t", id])?)
    }

    pub fn list_panes(session_name: &str, win_id: u16) -> Ret<String> {
        Ok(Self::run(&[
            "list-panes",
            "-t",
            &format!("{session_name}:{win_id}"),
            "-F",
            "#{pane_index}|#{pane_id}|#{pane_active}|#{pane_width}|#{pane_height}|#{pane_current_command}|#{pane_pid}",
        ])?)
    }

    pub fn capture_pane(session_name: &str, win_id: u16, pane_id: u16) -> Ret<String> {
        Ok(Self::run(&[
            "capture-pane",
            "-p",
            "-t",
            &format!("{session_name}:{win_id}.{pane_id}"),
        ])?)
    }

    pub fn capture_pane_id(id: &str) -> Ret<String> {
        Ok(Self::run(&["capture-pane", "-p", "-t", id])?)
    }

    pub fn get_cmd(pid: u64) -> Ret<String> {
        let command = Command::new("ps")
            .args(["--ppid", &pid.to_string(), "-o", "args="])
            .output()?;
        if command.status.success() {
            Ok(String::from_utf8_lossy(&command.stdout).trim().to_string())
        } else {
            Ok(String::from_utf8_lossy(&command.stderr).trim().to_string())
        }
    }
}
