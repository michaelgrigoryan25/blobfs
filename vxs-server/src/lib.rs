pub mod rpc_queue;

/// [VxServer] is a server implementation for vxs. It returns a struct,
/// which provides different useful handlers, including for starting,
/// stopping, etc.
pub struct VxServer {
    /// vxs-server address must be a valid [ip address][ip-wiki], or [hostname][host-wiki].
    ///
    /// [ip-wiki]: <https://en.wikipedia.org/wiki/IP_ad>    
    /// [host-wiki]: <https://en.wikipedia.org/wiki/Hostname>
    pub(crate) address: String,

    /// vxs-server can function in a verbose mode, in case of which, everything, including
    /// non-necessary information for production will be printed to stdout and written to
    /// a dedicated log file.
    pub(crate) verbose: Box<bool>,
}

impl VxServer {
    /// Creates a new [VxServer] with the provided configuration. The variables must be
    /// stored in the heap and passed as Box pointers.
    pub fn new(address: String, verbose: Box<bool>) -> VxServer {
        VxServer { address, verbose }
    }
}
