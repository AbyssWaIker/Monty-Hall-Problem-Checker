use std::sync::mpsc;
use std::thread;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Number of tries to test each decision 
   #[arg(short, long, default_value_t = 1_000_000_000)]
   number_of_tests: u64,

   /// Number of threads to use for tests
   #[arg(short, long, default_value_t = 8)]
   threads: u16,
}


fn choose_door(b_does_viewer_switches_the_door: bool) -> bool {
    let correct_door: u8 = rand::random::<u8>() % 3;
    let chosen_door: u8 = rand::random::<u8>() % 3;

    let b_had_viewer_chosen_correctly = chosen_door == correct_door;
    let b_had_viewer_won = b_had_viewer_chosen_correctly != b_does_viewer_switches_the_door;
    b_had_viewer_won
}
fn repeat_tests_with_set_switch(
    number_of_tests: u64,
    b_does_viewer_switches_the_door: bool,
) -> u64 {
    let mut n_successes: u64 = 0;
    for _i in 0..number_of_tests {
        n_successes += choose_door(b_does_viewer_switches_the_door) as u64;
    }
    n_successes
}

fn repeat_tests_with_set_switch_threaded(
    number_of_tests: u64,
    b_does_viewer_switches_the_door: bool,
    n_threads:u64
) -> u64 {
    let (thread_sender, thread_receiver) = mpsc::channel();

    for thread_id in 0..n_threads {
        let number_of_tests_for_the_thread: u64 = if thread_id == 0 {
            number_of_tests / n_threads + number_of_tests % n_threads
        } else {
            number_of_tests / n_threads
        };

        let local_thread_sender = thread_sender.clone();
        thread::spawn(move || {
            let n_successes: u64 = repeat_tests_with_set_switch(
                number_of_tests_for_the_thread,
                b_does_viewer_switches_the_door,
            );
            local_thread_sender.send(n_successes)
        });
    }
    // We can close this Sender
    drop(thread_sender);
    thread_receiver.iter().sum()
}


fn main() {
    let args = Args::parse();

    let result_for_yes_switch = repeat_tests_with_set_switch_threaded(args.number_of_tests, true, args.threads as u64);
    println!("With switch, \t{result_for_yes_switch}/{}", args.number_of_tests);
    let result_for_not_switch = repeat_tests_with_set_switch_threaded(args.number_of_tests, false, args.threads as u64);
    println!("Without switch \t{result_for_not_switch}/{}", args.number_of_tests);
}
