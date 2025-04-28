# Pressure
The tool to gather system performance metrics useful for low latency systems.
In general it can help to get system performance baselines for a specific hardware/software configuration.
A large asterisk is that each run can vary due too a non exhaustive list such as - CPU power states, other system activity and various of soft/hard interrupts

Current features:
1. **Core Latency**: Measure the latency between CPU cores using a CAS operation.
2. **Tick Resolution**: Measure the resolution of various system timers and calculates characteristics like the CPU's Timestamp Counter (TSC) frequency.
3. **Core noise**: Measure how much jitter their is on a specific core when processing data in a tight loop

These features are useful for benchmarking and analyzing CPU behavior, particularly in systems with multiple cores.
## How It Works
### 1. **Core Latency**
- The program calculates the communication latency between every pair of CPU cores on the system, using specific threads pinned to individual cores.
- Each pair of cores that are being measured - will have 3 runs done where average one way trip is measured. The median run will be presented.

### 2. **Tick Resolution**
- Measures the level of detail (LOD) provided by various timing mechanisms, such as:
    - **RDTSC**: Instruction-level timer using the CPU's Timestamp Counter.
    - **RDTSCP**: A more serialized version of RDTSC for more dependable results.
    - **System Clock**: High-level timing provided by the operating system.

- Retrieves TSC frequency and converts a tick into nanoseconds.

### 3. **Core noise**
- Busy loop on each cpu core - and measure how often it takes more than a certain threshold (default 200 nanos) to retrieve a new timestamp counter.

## How to Run
1. **Build and Run**: Use `cargo build --release` to compile the program, then run it with the provided executable.
```
   ./pressure cl
```
The `all` argument is mandatory and ensures you're in the correct mode.
1. **CPU Latency Output**: A matrix of latencies is printed with core IDs as headers. Latency values are in nanoseconds.
2. **Tick Resolution Output**: The resolution of the various timing mechanisms is measured and displayed, along with the CPU's TSC frequency.
3. **Core noise Output**: A per core output stating how much of the time it spends processing outside events (ie. interrupts, other processes etc.).

## Example Output
```
Welcome to pressure measurement tool.
----------------------------------------
Validating...
Validation done
----------------------------------------
MACHINE INFORMATION
System have 16 cores available
Cpu brand(s): 13th Gen Intel(R) Core(TM) i5-13400F
Total physical memory: 32581 MB
Total swap memory: 2048 MB
----------------------------------------
Measuring core latency...
  CPU    0    1    2    3    4    5    6    7    8    9   10   11   12   13   14   15
    0
    1    5
    2   32   38
    3   40   42    5
    4   43   42   41   41
    5   38   38   36   38    5
    6   34   38   35   43   36   36
    7   36   40   41   37   39   37    5
    8   33   37   35   32   40   33   39   32
    9   36   37   29   36   39   30   31   35    5
   10   37   35   38   29   38   34   38   36   33   28
   11   30   42   37   34   39   34   34   37   34   30    5
   12   53   68   54   54   49   52   51   55   52   47   55   49
   13   54   53   59   52   54   51   57   52   60   50   61   57   79
   14   48   66   62   51   51   52   50   54   58   51   52   67  103   81
   15   52   56   54   55   50   49   50   56   48   56   51   52   88   74   80
----------------------------------------
Measuring tick resolution...
RDTSC   LOD is     7 nanos derived from 18 ticks observed on median
RDTSCP  LOD is    11 nanos derived from 27 ticks observed on median
Clock   LOD is   100 nanos derived from non-zero measurements
TSC     1 tick = 0.4 nanos - TSC frequency = 2496.0 MHz

----------------------------------------
Measuring core noise...
Measuing core noise - with tolerance of 200 nanos (ie. collect measurements above this number)
CPU 0 - Total noise: 38420 micros - corresponding to 2.71% of total runtime
CPU 1 - Total noise: 39549 micros - corresponding to 2.78% of total runtime
CPU 2 - Total noise: 62722 micros - corresponding to 3.96% of total runtime
CPU 3 - Total noise: 18152 micros - corresponding to 1.19% of total runtime
CPU 4 - Total noise: 10723 micros - corresponding to 0.76% of total runtime
CPU 5 - Total noise: 7444 micros - corresponding to 0.55% of total runtime
CPU 6 - Total noise: 7414 micros - corresponding to 0.55% of total runtime
CPU 7 - Total noise: 9344 micros - corresponding to 0.68% of total runtime
CPU 8 - Total noise: 7259 micros - corresponding to 0.55% of total runtime
CPU 9 - Total noise: 8090 micros - corresponding to 0.61% of total runtime
CPU 10 - Total noise: 10150 micros - corresponding to 0.77% of total runtime
CPU 11 - Total noise: 8709 micros - corresponding to 0.66% of total runtime
CPU 12 - Total noise: 142363 micros - corresponding to 3.50% of total runtime
CPU 13 - Total noise: 92625 micros - corresponding to 2.35% of total runtime
CPU 14 - Total noise: 62180 micros - corresponding to 1.60% of total runtime
CPU 15 - Total noise: 28876 micros - corresponding to 0.76% of total runtime
----------------------------------------

```
## Requirements
- **Rust And Cargo**: Besides Cargo and Rust - for full support it assumes x86 platform either on AMD or Intel CPU set 

## TODO

Identify interruptions on relevant CPUs
    a. printout of suggestion to lower/remove interruptions
