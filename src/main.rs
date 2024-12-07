extern crate core_affinity;

use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::time::Instant;
use std::{env, thread};
use std::thread::JoinHandle;
use core_affinity::CoreId;
use sysinfo::{System};

const RTT_COUNT: u32 = 50_000_000;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() < 4 || args[1] != "cl" && args[1] != "cpulatency" {
        eprintln!("Usage: {} cl <core1> <core2>", args[0]);
        return;
    }

    let (number1, number2) = match extract_arguments(args) {
        Some(value) => value,
        None => return,
    };

    println!("Will measure latency between CPUs: {} and {}", number1, number2);

    let (core1, core2) = match validate_and_select_cores(number1, number2) {
        Some(value) => value,
        None => return,
    };

    // Use Arc to safely share flag across threads
    let flag = Arc::new(AtomicBool::new(false));

    // Start timing
    let start = Instant::now();

    // First thread
    let flag_clone1 = Arc::clone(&flag);
    let flag_clone2 = Arc::clone(&flag);
    let handle1 = create_round_trip_thread(core1, RTT_COUNT, flag_clone1, false);
    let handle2 = create_round_trip_thread(core2, RTT_COUNT, flag_clone2, true);

    handle1.join().unwrap();
    handle2.join().unwrap();

    let elapsed = start.elapsed();
    let ops_time = elapsed.as_nanos() as f64 / (RTT_COUNT * 2) as f64;
    println!("Total time for {} ping-pong operations: {:?} corresponding to: {:.1} nanos / single trip", RTT_COUNT, elapsed, ops_time);

    retrieve_cpu_clock_info();
}

fn retrieve_cpu_clock_info() {
    // Create a System object
    let mut system = System::new_all();
    // Refresh system information, including processor statistics
    system.refresh_all();

    // Access CPU processor list
    let processors = system.cpus();

    for cpu in processors {
        let frequency = cpu.frequency();
        println!("Current CPU frequency: {} MHz", frequency);
    }
}

fn validate_and_select_cores(number1: usize, number2: usize) -> Option<(CoreId, CoreId)> {
    // Get list of cores
    let cores = core_affinity::get_core_ids().unwrap();
    if cores.len() < number1
        || cores.len() < number2 {
        eprintln!("The specified CPU cores is out of range");
        return None;
    }

    let core1 = cores[number1];
    let core2 = cores[number2];
    Some((core1, core2))
}

fn create_round_trip_thread(core: CoreId, iterations: u32, flag_clone: Arc<AtomicBool>, start_state: bool) -> JoinHandle<()> {
    thread::spawn(move || {
        core_affinity::set_for_current(core);
        for _ in 0..iterations {
            while flag_clone.compare_exchange_weak(start_state, !start_state, Ordering::SeqCst, Ordering::SeqCst).is_err() {}
        }
    })
}

fn extract_arguments(args: Vec<String>) -> Option<(usize, usize)> {
    let number1: usize = match args[2].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("The core id 1 is not a valid integer.");
            return None;
        }
    };

    let number2: usize = match args[3].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("The core id 2 is not a valid integer.");
            return None;
        }
    };

    if number1 == number2 {
        eprintln!("The cpu ids cannot be the same.");
        return None;
    }

    Some((number1, number2))
}