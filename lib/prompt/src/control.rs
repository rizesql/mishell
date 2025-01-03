use mishell_core::sys;

use crate::error::Error;

pub struct TerminalControl {
    prev_fg_pid: Option<nix::unistd::Pid>,
}

impl TerminalControl {
    pub fn acquire() -> Result<Self, Error> {
        let prev_fg_pid = sys::terminal::get_fg_pid();

        sys::signal::lead_new_pgrp()?;
        sys::terminal::move_self_to_fg()?;

        sys::signal::mask_sigttou()?;

        Ok(Self { prev_fg_pid })
    }

    pub fn try_release(&mut self) {
        if let Some(pid) = self.prev_fg_pid {
            if sys::terminal::move_to_fg(pid).is_ok() {
                self.prev_fg_pid = None;
            }
        }
    }
}

impl Drop for TerminalControl {
    fn drop(&mut self) {
        self.try_release()
    }
}
