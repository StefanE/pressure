use crate::tick_resolution;

use std::arch::x86_64::_rdtsc;
const SAMPLES: u64 = 100_000_000;
pub fn core_noise(tolerance: u64) {
    let core_ids = core_affinity::get_core_ids().expect("Failed to get core IDs");
    let nano_per_tick = tick_resolution::get_nanosecond_per_tick();
    let tick_tolerance = (tolerance as f64 / nano_per_tick) as u64;

    println!("Measuing core noise - with tolerance of {} nanos (ie. collect measurements above this number)", tolerance);

    // Ensure all pairs of CPU cores are tried
    for (_, &core1) in core_ids.iter().enumerate() {
        core_affinity::set_for_current(core1);
        unsafe {
            let mut noise_sum = 0;
            let t1 = _rdtsc();
            for _ in 0..SAMPLES {
                let t1 = _rdtsc();
                let t2 = _rdtsc();
                if t2 - t1 > tick_tolerance {
                    noise_sum += (t2 - t1) - tick_tolerance;
                }
            }
            let t2 = _rdtsc();

            let total = t2 - t1;
            let interruption_percentage = (noise_sum as f64 / total as f64) * 100f64;
            let lost_time = (noise_sum as f64 * nano_per_tick) / 1000f64;
            println!("CPU {} - Total noise: {:.0} micros - corresponding to {:.2}% of total runtime", &core1.id, lost_time, interruption_percentage)
        }
    }

}
