extern crate core_affinity;
mod core_latency;
mod core_noise;
mod tick_resolution;
use raw_cpuid::CpuId;
use std::collections::HashSet;
use std::env;
use std::time::Instant;
use sysinfo::System;

// Constants
const RTT_COUNT: u32 = 1_000_000;
const DEFAULT_NOISE_TOLERANCE: u64 = 200;
const VALID_MODE: &str = "all";

const SPLIT_STRING: &str = "----------------------------------------";

fn main() {
    let start_time = Instant::now();
    let args: Vec<String> = env::args().collect();
    if let Err(msg) = validate_args(&args) {
        eprintln!("{}", msg);
        return;
    }

    println!("Welcome to pressure measurement tool.");
    println!("{}", SPLIT_STRING);
    validate_machine();
    println!("{}", SPLIT_STRING);
    print_machine_info();

    println!("{}", SPLIT_STRING);
    println!("Measuring core latency...");
    core_latency::core_latency();
    println!("{}", SPLIT_STRING);
    println!("Measuring tick resolution...");
    tick_resolution::do_tick_resolution_measurement();
    println!();
    println!("{}", SPLIT_STRING);
    println!("Measuring core noise...");
    core_noise::core_noise(DEFAULT_NOISE_TOLERANCE);
    println!("{}", SPLIT_STRING);

    let end_time = Instant::now();
    println!(
        "Measurements took {:.3} seconds.",
        (end_time - start_time).as_secs_f32()
    );
}

fn validate_args(args: &[String]) -> Result<(), &'static str> {
    if args.len() < 2 || args[1] != VALID_MODE {
        return Err("Usage: pressure all");
    }
    Ok(())
}


fn validate_machine() {
    println!("Validating...");
    let cpuid = CpuId::new();
    if let Some(feature_info) = cpuid.get_advanced_power_mgmt_info() {
        if !feature_info.has_invariant_tsc() {
            eprintln!("WARN: The CPU does NOT support Invariant TSC. This may cause problems.");
        }
    } else {
        panic!("Unable to retrieve feature information from the CPU.");
    }

    println!("Validation done");
}

fn print_machine_info() {
    println!("MACHINE INFORMATION");
    let cores = core_affinity::get_core_ids().expect("Could not get core IDs.");
    println!("System have {} cores available", cores.len());

    let mut sys = System::new_all();
    sys.refresh_all();

    // Find and print all unique processor brands
    let mut unique_brands = HashSet::new();
    for processor in sys.cpus() {
        unique_brands.insert(processor.brand().to_string());
    }

    println!("Cpu brand(s): {}",
             unique_brands
                 .into_iter()
                 .collect::<Vec<_>>() // Collect into a vector to join
                 .join(", ") // Use join for comma-separated output
    );

    println!(
        "Total physical memory: {} MB",
        sys.total_memory() / (1024 * 1024)
    );
    println!(
        "Total swap memory: {} MB",
        sys.total_swap() / (1024 * 1024)
    );
}