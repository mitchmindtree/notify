extern crate notify_backend as backend;

extern crate notify_backend_poll_tree;

#[cfg(any(
    target_os = "linux",
    target_os = "android",
))]
extern crate notify_backend_inotify as inotify;

#[cfg(any(
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd",
))]
extern crate notify_backend_kqueue as kqueue;

use backend::prelude::*;

fn new_backend() -> BackendResult<Box<NotifyBackend<Item=StreamItem, Error=StreamError>>> {
    Err(BackendError::NotImplemented)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
