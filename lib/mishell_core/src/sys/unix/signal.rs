use crate::error;

pub fn lead_new_pgrp() -> Result<(), error::Error> {
    nix::unistd::setpgid(nix::unistd::Pid::from_raw(0), nix::unistd::Pid::from_raw(0))?;
    Ok(())
}

pub fn mask_sigttou() -> Result<(), error::Error> {
    let ignore = nix::sys::signal::SigAction::new(
        nix::sys::signal::SigHandler::SigIgn,
        nix::sys::signal::SaFlags::empty(),
        nix::sys::signal::SigSet::empty(),
    );

    unsafe { nix::sys::signal::sigaction(nix::sys::signal::SIGTTOU, &ignore) }?;
    Ok(())
}
