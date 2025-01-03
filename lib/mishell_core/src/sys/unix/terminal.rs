use std::io::IsTerminal;

use crate::error;

pub fn get_fg_pid() -> Option<nix::unistd::Pid> {
    nix::unistd::tcgetpgrp(std::io::stdin()).ok()
}

pub fn move_to_fg(pid: nix::unistd::Pid) -> Result<(), error::Error> {
    nix::unistd::tcsetpgrp(std::io::stdin(), pid)?;
    Ok(())
}

pub fn move_self_to_fg() -> Result<(), error::Error> {
    if std::io::stdin().is_terminal() {
        let pgid = nix::unistd::getpgid(None)?;
        let _ = move_to_fg(pgid);
    }

    Ok(())
}
