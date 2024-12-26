use std::time::Instant;

fn log_thing(total_threads: u32) -> u32 {
    return total_threads - (f64::log(total_threads.into(), 5.0).round() as u32);
}

fn my_thing(total_threads: u32) -> u32 {
    if total_threads >= 32 {
        return total_threads - 4;
    } else if total_threads >= 12 {
        return total_threads - 2;
    } else if total_threads >= 3 {
        return total_threads - 1;
    } else {
        return total_threads;
    }

    //println!("{}", max_threads)
}

//     let mut total_threads = thread::available_parallelism().unwrap().get() as u32;

fn main() {
    /*
    let now = Instant::now();
    for _ in 0..100000000 {

    }
    let elapsed = now.elapsed();
    */
    let total_threads: u32 = 128;
    println!("{}", log_thing(total_threads));
    println!("{}", my_thing(total_threads));
    //println!("{}", elapsed.as_nanos());
}
