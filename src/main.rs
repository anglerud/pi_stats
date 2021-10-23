use std::fmt;
use std::io::{self, Write};
use std::{thread, time};

use sysinfo::{ComponentExt, ProcessorExt, System, SystemExt};

#[derive(Debug)]
struct PiStats {
    /// CPU frequency, core average presumably.
    cpu_frequency: u64,
    /// This is the composite temperature, a
    temperature: f32,
    /// Available memory
    /// Note that this is different from 'free' memory in that this
    /// takes into account disk cache and buffers that the OS will
    /// reclaim under pressure.
    memory_available_kb: u64,
}

impl fmt::Display for PiStats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} Mhz / {} C / {} kB",
            self.cpu_frequency, self.temperature, self.memory_available_kb
        )
    }
}

/// Returns the composite temperature
/// The composite appears to be
/// Composite: 41.85°C (max: 69.85°C / critical: 89.85°C)
/// MAC Temperature: 67.772°C (max: 67.894°C)
/// PHY Temperature: 69.164°C (max: 69.222°C)
/// Tccd1: 40.25°C (max: 44.75°C)
/// Tctl: 49.875°C (max: 58°C)
/// Tdie: 49.875°C (max: 58°C)
/// CPU: 16.8°C (max: 16.8°C)
fn temperature(sys: &mut System) -> Option<f32> {
    // I gotta say, the sysinfo 'components' API is... suboptimal.
    // There's a few other libraries like `heim` I should probably
    // check out instead.
    // We may want to get "CPU" instead here, but on my machine that
    // appears to be at like 16C, which is clearly not the CPU die.
    for component in sys.components() {
        if component.label() == "Composite" {
            // This is what I need for my desktop machine.
            return Some(component.temperature());
        } else if component.label() == "CPU" {
            // This is needed for the Raspberry PI.
            return Some(component.temperature());
        }
    }

    None
}

fn stats(sys: &mut System) -> PiStats {
    PiStats {
        cpu_frequency: sys.global_processor_info().frequency(),
        temperature: temperature(sys).unwrap_or(0.0),
        memory_available_kb: sys.available_memory(),
    }
}

fn main() {
    let one_second = time::Duration::from_secs(1);
    let mut sys = System::new_all();

    loop {
        let stats = stats(&mut sys);
        print!("\x1b[2K\r"); // Escape code (27 in hex), clear line.
        print!("{}\r", stats);
        io::stdout().flush().unwrap();

        thread::sleep(one_second);
        sys.refresh_all();
    }
}
