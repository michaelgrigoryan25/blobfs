use std::{fmt::Debug, future::Future};

use log::debug;

/// Custom unsafe wrapper around [Send], used for passing around pointers
/// between [tokio::task]s.
pub(crate) struct UnsafeSend<T>(T);

/// The Send trait is automatically implemented when the compiler determines it's appropriate.
/// However, for current use-case, it would not be possible, since it is not allowed to share
/// multiple mutable references between threads.
///
/// The values must also be `'static` for usage with tokio::spawn, which is not really necessary
/// for our use-case.
unsafe impl<T> Send for UnsafeSend<T> {}

impl<T: Future> Future for UnsafeSend<T> {
    type Output = T::Output;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        unsafe { self.map_unchecked_mut(|i| &mut i.0).poll(cx) }
    }
}

/// ## Description
///
/// This function spawns a separate asynchronous [tokio::task], and awaits for `ctrl+c` signal in
/// the background. After capturing the signal, source value will be modified and be set to the
/// target value.
///
/// ## Safety
///
/// Despite the fact that this function contains `unsafe` code, it is completely safe to use. Just
/// be sure that the provided arguments are non-null pointers, since the function will not handle
/// this for you.
///
/// ## Panics
///
/// This function does not panic.
///
/// ## Usage
///
/// Below, is a simple example of a while loop, that will be gracefully terminated
/// after `ctrl+c`.
///
/// ```rs
/// let (mut run, target) = (true, false);
/// graceful::shutdown(&mut run, &target);
/// while run {}
/// ```
pub fn shutdown<T>(source: *mut T, target: *const T)
where
    T: Send + Copy + Debug + 'static,
{
    // Spawning a separate asynchronous tokio thread in the background to listen
    // for the signal.
    tokio::spawn(UnsafeSend(async move {
        unsafe {
            if tokio::signal::ctrl_c().await.is_ok() {
                // If Ctrl + C was captured successfully, updating the source value with the target.
                *source = *target;
                // Additionally, logging some information to stdout, useful for debugging.
                debug!(target: "graceful::shutdown", "ctrl+c received. source set to: {:?}", &*target);
            }
        }
    }));
}
