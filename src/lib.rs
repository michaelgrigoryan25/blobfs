use backtrace::Backtrace;

#[macro_use]
extern crate log;

/// Contains various commands, their arguments, for working with the command-line.
pub mod cmd;
/// Core server code is included in this module.
pub mod server;

/// Simple wrapper around [Result], where we don't care about the received
/// error type.
///
/// Note that this wrapper only works with errors which implement [std::error::Error].
pub type Unspecified<T> = Result<T, Box<dyn std::error::Error>>;

/// Bootstraps, and starts a new vxs server with the provided [cmd::VxCommandStartArgs].
///
/// ## Panic behavior
///
/// This function does not panic, however, if the logger fails to initialize
/// the program will print current backtrace, and error to stderr, and will
/// invoke [std::process::exit] with status code `1` in the end.
pub async fn bootstrap(args: cmd::VxCommandStartArgs) -> Unspecified<()> {
    server::Server::new(args.address)
        .run(Some(Box::new(move |addr| {
            if let Err(e) = || -> Unspecified<()> {
                let mut logger = fern::Dispatch::new().format(|out, msg, record| {
                    out.finish(format_args!(
                        "{} ({}) {}",
                        chrono::Local::now().format("%Y-%m-%d@%H:%M:%S"),
                        record.level().as_str().to_lowercase(),
                        msg,
                    ));
                });

                if args.verbose {
                    logger = logger.level(log::LevelFilter::Trace);
                }

                logger
                    .chain(std::io::stdout())
                    .chain(fern::log_file("debug.log")?)
                    .apply()?;

                Ok(())
            }() {
                eprintln!("fern: cannot initialize the logger");
                eprintln!("original error: {}", e);
                eprintln!("{}", divider("BACKTRACE"));
                eprintln!("{:#?}", Backtrace::new());
                std::process::exit(1);
            }

            print!("{}\n", divider("VXS"));
            debug!("fern: logger initialized");
            info!("tcp server started at: {}", addr);
        })))
        .await?;

    Ok(())
}

#[inline(always)]
#[doc(hidden)]
/// There is no need pf creating a separate stack frame just for
/// this small function, thus, it can be inlined.
fn divider(msg: &str) -> String {
    format!("{d} {m} {d}", d = "-".repeat(30), m = msg)
}
