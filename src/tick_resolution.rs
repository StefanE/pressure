use std::arch::x86_64::{__rdtscp, _rdtsc};
use std::time::Instant;
use raw_cpuid::CpuId;

const ITERATIONS: u128 = 10_000_000;

pub fn do_tick_resolution_measurement() {

    
    measurement_high_resolution();
    measurement_high_resolution_rdtscp();
    measurement_low_resolution();

    let cpuid = CpuId::new();
    let tsc_info = cpuid.get_tsc_info().expect("Could not get TSC info.");
    println!(
        "TSC\t1 tick = {:.1} nanos - TSC frequency = {:.1} MHz",
        1_000_000_000.0 / tsc_info.tsc_frequency().unwrap() as f64,
        tsc_info.tsc_frequency().unwrap() as f64 / 1_000_000.0
    );
}

pub fn get_nanosecond_per_tick() -> f64 {
    let cpuid = CpuId::new();
    let tsc_info = cpuid.get_tsc_info().expect("Could not get TSC info.");
    1_000_000_000.0 / tsc_info.tsc_frequency().unwrap() as f64
}

fn measurement_high_resolution() {
    unsafe {
        let mut vec: Vec<u64> = Vec::with_capacity(ITERATIONS as usize);
        let start_time = Instant::now();
        let start_ticks = _rdtsc();
        for _ in 0..ITERATIONS {
            let t1 = _rdtsc();
            let t2 = _rdtsc();
            let delta = t2 - t1;
            vec.push(delta);
        }
        let end_ticks = _rdtsc();
        let end_time = Instant::now();
        vec.sort_by(|a, b| a.cmp(b));
        let ticks_per_nano =
            (end_ticks - start_ticks) as f64 / (end_time - start_time).as_nanos() as f64;
        let idx = (ITERATIONS / 2) as usize;
        println!(
            "RDTSC\tLOD is {:>5.0} nanos derived from {} ticks observed on median",
            (vec[idx] as f64) / ticks_per_nano,
            vec[idx]
        );
    }
}

fn measurement_high_resolution_rdtscp() {
    unsafe {
        let mut vec: Vec<u64> = Vec::with_capacity(ITERATIONS as usize);
        let start_time = Instant::now();
        let mut aux = 0u32;
        let start_ticks = __rdtscp(&mut aux);
        for _ in 0..ITERATIONS {

            let t1 = __rdtscp(&mut aux);
            let t2 = __rdtscp(&mut aux);
            let delta = t2 - t1;
            vec.push(delta);
        }
        let end_ticks = __rdtscp(&mut aux);
        let end_time = Instant::now();
        vec.sort_by(|a, b| a.cmp(b));
        let ticks_per_nano =
            (end_ticks - start_ticks) as f64 / (end_time - start_time).as_nanos() as f64;
        let idx = (ITERATIONS / 2) as usize;
        println!(
            "RDTSCP\tLOD is {:>5.0} nanos derived from {} ticks observed on median",
            (vec[idx] as f64) / ticks_per_nano,
            vec[idx]
        );
    }
}

fn measurement_low_resolution() {
    let mut vec: Vec<u64> = Vec::with_capacity(ITERATIONS as usize);
    for _ in 0..ITERATIONS {
        let t1 = Instant::now();
        let t2 = Instant::now();
        let delta_time = (t2 - t1).as_nanos();
        vec.push(delta_time as u64);
    }
    vec.sort_by(|a, b| a.cmp(b));
    let non_zero = vec.iter().find(|&x| *x > 0).unwrap();
    println!(
        "Clock\tLOD is {:>5.0} nanos derived from non-zero measurements",
        non_zero
    );
}

