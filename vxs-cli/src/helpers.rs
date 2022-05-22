pub mod proc {
    use std::time::Duration;

    use sysinfo::{Pid, Process, ProcessExt, SystemExt};

    /// Prints a list of `n` processes currently running vxs-server. If limit is [None]
    /// all processes running vxs-server will be printed to stdout.
    pub async fn list(args: crate::VxCommandProcArgs) -> Result<(), Box<dyn std::error::Error>> {
        let mut sys = sysinfo::System::new();
        let dur = Duration::from_secs(args.interval);

        loop {
            sys.refresh_all();

            // Only using processes which include the substring `vxs`.
            let processes = sys
                .processes()
                .iter()
                .filter(|(_, process)| process.name().contains("vxs"))
                .collect::<Vec<(&Pid, &Process)>>();

            let processes = if let Some(limit) = args.limit {
                &processes[..=limit]
            } else {
                &processes
            };

            for (pid, process) in processes {
                println!(
                    "name: {} | pid: {pid} | memory usage: {} | cpu usage: {:.2} | task: {} | uptime: {}",
                    process.name(),
                    process.memory().to_string(),
                    process.cpu_usage(),
                    process.exe().display(),
                    process.run_time()
                )
            }

            // Continuous process information reporting requested.
            if args.continuous {
                tokio::time::sleep(dur).await;
                // Clearing the contents of the terminal.
                clearscreen::clear()?;
            } else {
                // If the `continuous` flag was not specified, breaking the loop
                // after the first iteration.
                return Ok(());
            }
        }
    }
}
