#[macro_use]
extern crate log;

pub(crate) mod metrics;
pub(crate) mod router;

/// [VxServer] is a server implementation for vxs. It returns a struct,
/// which provides different useful handlers, including for starting,
/// stopping, etc.
pub struct VxServer {
    /// vxs-server address must be a valid [ip address][ip-wiki], or [hostname][host-wiki].
    ///
    /// [ip-wiki]: <https://en.wikipedia.org/wiki/IP_ad>    
    /// [host-wiki]: <https://en.wikipedia.org/wiki/Hostname>
    #[allow(dead_code)]
    pub(crate) address: String,

    /// vxs-server can function in a verbose mode, in case of which, everything, including
    /// non-necessary information for production will be printed to stdout and written to
    /// a dedicated log file.
    #[allow(dead_code)]
    pub(crate) verbose: Box<bool>,
}

impl VxServer {
    /// Creates a [hyper::Server] from the provided address, initializes the logger, and starts
    /// the server in a separate [tokio] thread. If running in detached mode, the function
    /// will only return a handle to the thread where the server runs.
    pub async fn create(_: vxs_cli::VxCommandStartArgs) -> anyhow::Result<()> {
        VxServer::init()?;
        unimplemented!()
    }

    /// Initializes everything needed for the server to function, including
    /// logging, persistence directories, configuration, etc.
    pub(crate) fn init() -> anyhow::Result<()> {
        fern::Dispatch::new()
            .format(|out, msg, rec| {
                out.finish(format_args!(
                    "[{}] {} {}: {}",
                    rec.level(),
                    chrono::Local::now(),
                    rec.target(),
                    msg
                ))
            })
            .level(log::LevelFilter::Info)
            .chain(std::io::stdout())
            .apply()?;

        info!(target: "VxServer::init()", "fern initialized.");

        Ok(())
    }
}
