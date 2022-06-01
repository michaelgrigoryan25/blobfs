use std::time::Duration;

use sysinfo::{Pid, Process, ProcessExt, SystemExt};
use vxs_common::graceful;

use crate::{cmd::VxCommandProcArgs, UnspecifiedError};

/// Prints a list of `n` processes currently running vxs. If limit is [None]
/// all processes running vxs-server will be printed to stdout.
pub async fn list(args: VxCommandProcArgs) -> UnspecifiedError<()> {
    let mut run = true;
    // Starting a background task, and listening for ctrl+c. `run`
    // will be updated, and set to `false` after the capture.
    graceful::shutdown(&mut run, &false);

    let mut sys = sysinfo::System::new();
    // This is the interval that the user defined in seconds.
    let sleep_duration = Duration::from_secs(args.interval);

    // This loop, contains a mutable condition. Clippy however, thinks that
    // the provided value does not change, but it does. Just in a separate
    // background task that will update the value automatically whenever
    // ctrl+c signal is received.
    #[allow(clippy::while_immutable_condition)]
    while run {
        sys.refresh_all();

        // Only using processes which include the substring `vxs`.
        let processes = sys
            .processes()
            .iter()
            .filter(|(_, process)| process.name().contains("vxs"))
            .collect::<Vec<(&Pid, &Process)>>();

        let processes = if let Some(limit) = args.limit {
            &processes[..limit - 1]
        } else {
            &processes
        };

        for (pid, process) in processes {
            let output = format!(
                "| name: {} | pid: {pid} | memory usage: {} | cpu usage: {:.2} | task: {} | uptime: {} |",
                process.name(),
                process.memory(),
                process.cpu_usage(),
                process.exe().display(),
                process.run_time()
            );

            println!("{output}\n{}", "-".repeat(output.len()));
        }

        // Continuous process information reporting requested.
        if args.continuous {
            // Sleeping for the specified number of seconds.
            tokio::time::sleep(sleep_duration).await;
        } else {
            // If the `continuous` flag was not specified, breaking the loop
            // after the first iteration.
            return Ok(());
        }
    }

    Ok(())
}
