use std::{
    sync::{Arc, Mutex},
    thread,
    time::Instant,
};

fn is_prime(num: u64) -> bool {
    if num < 2 {
        return false;
    }

    for i in 2..=(num as f64).sqrt() as u64 {
        if num % i == 0 {
            return false;
        }
    }

    true
}

fn count_primes(start: u64, end: u64) -> u64 {
    let mut count = 0;
    for num in start..=end {
        if is_prime(num) {
            count += 1;
        }
    }

    count
}

fn main() {
    const RANGE_START: u64 = 1;
    const RANGE_END: u64 = 10_000_000;
    const THREAD_NUM: usize = 4;
    const STEP: u64 = (RANGE_END - RANGE_START + 1) / THREAD_NUM as u64;

    let now = Instant::now();

    let result = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for i in 0..THREAD_NUM as u64 {
        let result = Arc::clone(&result);

        let handle = thread::spawn(move || {
            let start = RANGE_START + i * STEP;
            let end = std::cmp::min(RANGE_END, start + STEP);
            let count = count_primes(start, end);

            let mut result = result.lock().unwrap();
            *result += count;
        });

        handles.push(handle)
    }

    for handle in handles {
        if let Err(e) = handle.join() {
            eprintln!("Error joining thread: {:?}", e);
        }
    }

    let elapsed_time = now.elapsed();

    println!(
        "Found {} in {} milliseconds",
        *result.lock().unwrap(),
        elapsed_time.as_millis()
    );
}
