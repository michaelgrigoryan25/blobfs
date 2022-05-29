use std::time::Duration;

use sysinfo::{Pid, Process, ProcessExt, SystemExt};
use vxs_common::graceful;

use crate::{cmd::VxCommandProcArgs, UnspecifiedError};

/// Prints a list of `n` processes currently running vxs. If limit is [None]
/// all processes running vxs-server will be printed to stdout.
pub async fn list(args: VxCommandProcArgs) -> UnspecifiedError<()> {
    let (mut run, target) = (true, false);
    graceful::shutdown(&mut run, &target);
    let mut sys = sysinfo::System::new();
    let dur = Duration::from_secs(args.interval);

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
            tokio::time::sleep(dur).await;
        } else {
            // If the `continuous` flag was not specified, breaking the loop
            // after the first iteration.
            return Ok(());
        }
    }

    Ok(())
}