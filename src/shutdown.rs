use tokio::sync::broadcast;

/// Listens for the server shutdown signal.
///
/// Shutdown is signalled using a `broadcast::Receiver`. Only a single value is
/// ever sent. Once a value has been sent via the broadcast channel, the server
/// should shutdown.
///
/// The `Shutdown` struct listens for the signal and tracks that the signal has
/// been received. Callers may query for whether the shutdown signal has been
/// received or not.
pub(crate) struct Shutdown {
	shutdown: bool,
	notify: broadcast::Receiver<()>,
}

impl Shutdown {
	pub(crate) fn new(notify: broadcast::Receiver<()>) -> Self {
		Self {
			shutdown: false,
			notify,
		}
	}

	pub(crate) fn is_shutdown(&self) -> bool {
		self.shutdown
	}

	pub(crate) async fn recv(&mut self) {
		if !self.shutdown {
			// Cannot receive a "lag error" as only one value is ever sent.
			let _ = self.notify.recv().await;
			self.shutdown = true;
		}
	}
}
