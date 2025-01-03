pub fn get_fg_pid() -> Option<nix::unistd::Pid> {
    None
}

pub fn move_to_fg(pid: nix::unistd::Pid) -> Result<(), error::Error> {
    Ok(())
}

pub fn move_self_to_fg() -> Result<(), error::Error> {
    Ok(())
}
