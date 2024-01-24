pub mod monitoring {
    use sysinfo::{Components, Disks, Networks, System};

    pub async fn watch_pc_health() {
        let mut sys = System::new_all();

        // First we update all information of our `System` struct.
        sys.refresh_all();

        println!("=> system:");
        // RAM and swap information:
        println!("total memory: {} bytes", sys.total_memory());
        println!("used memory : {} bytes", sys.used_memory());
        println!("total swap  : {} bytes", sys.total_swap());
        println!("used swap   : {} bytes", sys.used_swap());

        // Display system information:
        println!("System name:             {:?}", System::name());
        println!("System kernel version:   {:?}", System::kernel_version());
        println!("System OS version:       {:?}", System::os_version());
        println!("System host name:        {:?}", System::host_name());

        // Number of CPUs:
        println!("NB CPUs: {}", sys.cpus().len());

        // Display processes ID, name na disk usage:
        for (pid, process) in sys.processes() {
            println!("[{pid}] {} {:?}", process.name(), process.disk_usage());
        }

        // We display all disks' information:
        println!("=> disks:");
        let disks = Disks::new_with_refreshed_list();
        for disk in &disks {
            println!("{disk:?}");
        }

        // Network interfaces name, data received and data transmitted:
        let networks = Networks::new_with_refreshed_list();
        println!("=> networks:");
        for (interface_name, data) in &networks {
            println!(
                "{interface_name}: {}/{} B",
                data.received(),
                data.transmitted()
            );
        }

        // Components temperature:
        let components = Components::new_with_refreshed_list();
        println!("=> components:");
        for component in &components {
            println!("{component:?}");
        }
    }

    async fn check_memory() {}

    ///////////////////////// UPDATED VERSION BELOW

    pub async fn watch_pc_health(mut shutdown_receiver: tokio::sync::oneshot::Receiver<()>) {
        let (data_sender, mut data_receiver) = tokio::sync::mpsc::channel::<MonitoringData>(100);

        // Spawn a task for continuous monitoring
        let monitoring_task = tokio::spawn(monitoring_loop(data_sender, shutdown_receiver));

        while let Some(data) = data_receiver.recv().await {
            // Process monitoring data as needed
            println!("{:?}", data);
        }

        // Wait for the monitoring task to finish
        monitoring_task.await.expect("Monitoring task panicked");
    }

    async fn monitoring_loop(
        data_sender: tokio::sync::mpsc::Sender<MonitoringData>,
        mut shutdown_receiver: tokio::sync::oneshot::Receiver<()>,
    ) {
        let mut sys = System::new_all();

        loop {
            tokio::select! {
                _ = tokio::time::sleep(tokio::time::Duration::from_secs(5)) => {
                    // Update all information of our `System` struct.
                    sys.refresh_all();

                    // Send monitoring data to the receiver
                    let _ = data_sender.send(MonitoringData {
                        total_memory: sys.total_memory(),
                        used_memory: sys.used_memory(),
                        total_swap: sys.total_swap(),
                        used_swap: sys.used_swap(),
                        cpus: sys.cpus().len(),
                        processes: sys
                            .processes()
                            .into_iter()
                            .map(|(pid, process)| (pid, process.name().to_owned(), process.disk_usage()))
                            .collect(),
                        disks: Disks::new_with_refreshed_list().collect(),
                        networks: Networks::new_with_refreshed_list()
                            .into_iter()
                            .map(|(interface_name, data)| (interface_name, data.received(), data.transmitted()))
                            .collect(),
                        components: Components::new_with_refreshed_list().collect(),
                    });
                }
                _ = &mut shutdown_receiver => {
                    // Shutdown the monitoring loop when the shutdown signal is received
                    break;
                }
            }
        }
    }

    pub struct MonitoringData {
        pub total_memory: u64,
        pub used_memory: u64,
        pub total_swap: u64,
        pub used_swap: u64,
        pub cpus: usize,
        pub processes: Vec<(i32, String, sysinfo::DiskUsage)>,
        pub disks: Vec<sysinfo::Disk>,
        pub networks: Vec<(String, u64, u64)>,
        pub components: Vec<sysinfo::Component>,
    }

    pub async fn stop_monitoring() {
        let (shutdown_sender, shutdown_receiver) = tokio::sync::oneshot::channel::<()>();

        // Start monitoring
        let monitoring_task = monitoring::watch_pc_health(shutdown_receiver);

        // ... Do other work ...

        // Terminate monitoring
        shutdown_sender
            .send(())
            .expect("Failed to send shutdown signal");
        monitoring_task.await.expect("Monitoring task panicked");
    }
}
