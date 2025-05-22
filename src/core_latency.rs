use std::io::Write;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::thread::JoinHandle;
use std::time::Instant;
use core_affinity::CoreId;
use crate::RTT_COUNT;

pub fn core_latency() {
    let core_ids = core_affinity::get_core_ids().expect("Failed to get core IDs");
    let mut vector: Vec<Vec<u64>> = vec![vec![0; core_ids.len()]; core_ids.len()];

    print!("Measuring...");
    // Ensure all pairs of CPU cores are tried
    for (i, &core1) in core_ids.iter().enumerate() {
        print!("CPU {}...", core1.id);
        std::io::stdout().flush().expect("Problem flushing");
        for &core2 in &core_ids[i + 1..] {
            let mut latency_runs: Vec<u64> = vec![0;3];
            for i in 0..3 {
                let run_latency = measure_latency_between_cores(core1, core2);
                let latency = ((run_latency.as_nanos() / RTT_COUNT as u128) / 2) as u64;
                latency_runs[i] = latency;
            }
            // Take the run in the middle
            latency_runs.sort();
            vector[core2.id][core1.id] = latency_runs[1];
        }
    }

    print_header(&core_ids);
    for (row_idx, row) in vector.iter().enumerate() {
        print!("{:>5}", core_ids[row_idx].id);
        for &value in row {
            if value == 0 {
                print!("{:>5}", " ");
            } else {
                print!("{:>5}", value);
            }
        }
        println!();
    }
}

fn print_header(core_ids: &Vec<CoreId>) {
    print!("{:>5}", "CPU");
    for core_id in core_ids {
        print!("{:>5}", core_id.id);
    }
    println!();
}

fn measure_latency_between_cores(core1: CoreId, core2: CoreId) -> std::time::Duration {
    let flag = Arc::new(AtomicBool::new(false));

    // Timer start
    let start_time = Instant::now();

    // Create threads
    let handle1 = create_round_trip_thread(core1, RTT_COUNT, Arc::clone(&flag), false);
    let handle2 = create_round_trip_thread(core2, RTT_COUNT, flag, true);

    // Wait for threads to complete
    handle1.join().unwrap();
    handle2.join().unwrap();

    // Calculate elapsed time
    start_time.elapsed()
}

fn create_round_trip_thread(
    core: CoreId,
    iterations: u32,
    flag: Arc<AtomicBool>,
    start_state: bool,
) -> JoinHandle<()> {
    thread::spawn(move || {
        core_affinity::set_for_current(core);
        for _ in 0..iterations {
            while flag
                .compare_exchange_weak(
                    start_state,
                    !start_state,
                    Ordering::SeqCst,
                    Ordering::SeqCst,
                )
                .is_err()
            {}
        }
    })
}