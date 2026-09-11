/*!
pls main binary */
#![allow
(
    unknown_lints,
)]

unsafe fn domain()
{
    /*
    libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    // ignore SIGTSTP (ctrl-Z) for the shell itself
    libc::signal(libc::SIGTSTP, libc::SIG_IGN);
    libc::signal(libc::SIGQUIT, libc::SIG_IGN);
    */
}

fn main()
{
    unsafe
    {
        domain()
    }
}
