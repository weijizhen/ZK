
extern crate sysinfo;
use sysinfo::{ProcessorExt, System, SystemExt};

mod serialization;
mod shuffle;
mod shuffle_api;
mod simple_example;
mod two_chip;
use std::thread;

use serialization::serialization_main;
use shuffle::shuffle_main;
use shuffle_api::shuffle_api_main;
use simple_example::simple_example_main;
use two_chip::two_chip_main;

fn main() {
    use std::time::Instant;
    let start = Instant::now();

    let handle_1 = thread::spawn(move || {
        serialization_main();
    });

    let mut cpu_rates = vec![];
    let mut handles = vec![];
    for _ in 0..20 {
        let handle = thread::spawn(move || {
            shuffle_main();
            shuffle_api_main();
            simple_example_main();
            two_chip_main();
        });
        handles.push(handle);
        let mut system = System::new_all();
        system.refresh_all();
        let cpu_percent = system.get_global_processor_info().get_cpu_usage();
        // println!("CPU利用率: {:.2}%", cpu_percent);
        cpu_rates.push(cpu_percent);
    }

    handle_1.join().unwrap();
    for handle in handles {
        handle.join().unwrap();
    }

    let mut m=0.0;
    for rate in cpu_rates  {
        println!("CPU利用率: {:.2}%", rate);
        if rate>m {
            m=rate;
        }
    }
    println!("Max CPU利用率: {:.2}%", m);
    println!("total_time cost: {:?} ms", start.elapsed().as_millis());
}
