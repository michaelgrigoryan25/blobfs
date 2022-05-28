use crate::cmd::VxCommandStartArgs;

pub(crate) mod metrics;

/// [VxServer] is a server implementation for vxs. It returns a struct,
/// which provides different useful handlers, including for starting,
/// stopping, etc.
pub struct VxServer {
    /// This must be a valid [ip address][ip-wiki], or [hostname][host-wiki].
    ///
    /// [ip-wiki]: <https://en.wikipedia.org/wiki/IP_ad>    
    /// [host-wiki]: <https://en.wikipedia.org/wiki/Hostname>
    pub(crate) address: String,

    /// Detached indicates, whether the user spawned the server in the background
    /// or started it in the foreground. This is passed as an argument from the
    /// CLI.
    pub(crate) detached: bool,
}

