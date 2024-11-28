//! Valida dummy implementation of waiting for children with timeouts
//!
//! Since Valida is both a baremetal target, and has only one thread, it has no
//! real ability to manage child processes in the way that would allow applying
//! a timeout. So all we can really do is set the process running and do a
//! blocking wait until it is done.

#![allow(bad_style)]

use std::io;
use std::process::{Child, ExitStatus};
use std::time::Duration;

pub fn wait_timeout(child: &mut Child, _dur: Duration) -> io::Result<Option<ExitStatus>> {
    child.wait().map(Some)
}
