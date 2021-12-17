extern crate crossbeam;

use std::time::Instant;

const THREAD_NUMS: usize = 10;
const NUMS_PER_THREAD: usize = 10_000_000;
const NUM_ELEMENTS: usize = THREAD_NUMS * NUMS_PER_THREAD;

fn counter_threading(nums: &Vec<i64>) -> i64 {
    crossbeam::scope(|s| {
        let threads: Vec<crossbeam::thread::ScopedJoinHandle<i64>> =
            nums
                .chunks(NUMS_PER_THREAD)
                .map(|chunk| {
                    s.spawn(move |_| {
                        chunk.iter().fold(0, |acc, v| acc + v)
                    })
                })
                .collect();
                
        threads
            .into_iter()
            .map(|t| t.join().unwrap())
            .fold(0, |acc, v| acc + v)
    }).unwrap()
}

fn counter_sync(nums: &Vec<i64>) -> i64 {
    nums.iter().fold(0, |acc, v| acc + v)
}

fn main() {
        let nums = (0..NUM_ELEMENTS).map(|i| i as i64).collect::<Vec<i64>>();
        let sync_timer = Instant::now();
        let actual_value = counter_sync(&nums);
        let elapsed = sync_timer.elapsed();
        
        println!("Actual Value: {}", actual_value);
        println!("\nTime taken to solve:\n\t{} seconds\n\t{} milliseconds\n\t{} nanoseconds\n", elapsed.as_secs(), elapsed.as_millis(), elapsed.as_nanos());

        let nums = (0..NUM_ELEMENTS).map(|i| i as i64).collect::<Vec<i64>>();
        let thread_timer = Instant::now();
        let threaded_value = counter_threading(&nums);
        let elapsed = thread_timer.elapsed();

        println!("Value from threads: {}", threaded_value);
        println!("\nTime taken to solve:\n\t{} seconds\n\t{} milliseconds\n\t{} nanoseconds\n", elapsed.as_secs(), elapsed.as_millis(), elapsed.as_nanos());
        
        println!("Values match: {}", actual_value == threaded_value);
}
