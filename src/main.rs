use bruteforus::*;
use std::{ thread, io, io::Write };
use multiqueue;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::boxed::Box;
extern crate chrono;
extern crate colored;
extern crate tokio;
use colored::*;

fn main() {
    // Parse all args provided by usr
    let user_configs = parse_args();

    // Read wordlist file and create a vector of words
    let file_contents = get_file_contents(&user_configs.wordlist);
    let wordlist: Vec<&str> = file_contents.split("\n").collect();
    let progress_bar = indicatif::ProgressBar::new(wordlist.len() as u64);
    progress_bar.set_style(indicatif::ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} ({eta})")
        .progress_chars("░> "));
    progress_bar.println("Starting bruteforce attack");

    // Handling multithreading functionality
    // Sending Box object with String to multiqueue to overcome "Unknown Size at compile time"
    let (send, recv): (multiqueue::BroadcastSender<Box<String>>, multiqueue::BroadcastReceiver<Box<String>>) = multiqueue::broadcast_queue((user_configs.threads / 2) as u64);
    let mut handle: Vec<thread::JoinHandle<()>> = vec![]; /*  Vector to store thread handles  */
    let recv_stream = recv.add_stream();   /*  Required incase of multi-threads support */
    let busy_threads_count = Arc::new(AtomicUsize::new(0)); /*  Variable to have current number of busy threads  */


    let bruteforce = Arc::new(Brutef::new());
    


    // Create threads according to user config (default: 20)
    for _ in 0..user_configs.threads {
        let client = Arc::clone(&bruteforce);
        let busy_threads_count_clone = Arc::clone(&busy_threads_count);

        // Create a clone of multiqueue receiver for each thread indivdually
        let recv_stream_clone = recv_stream.clone();
        let pb = progress_bar.downgrade();

        // Spawn thread and push its handle to handle vector.
        handle.push(thread::spawn(move || { 
            for url in recv_stream_clone { 
                // Increment "number of busy threads" by 1
                busy_threads_count_clone.fetch_add(1, Ordering::SeqCst);
                
                // Request URL and got basic response. ( Not a full response object )
                        // '&*' in '&*target_url' is required to convert type Box<String> to &str
                let resp_basic = match client.request((&url).to_string()) {
                        Ok(r) => r,
                        Err(_er) => {
                            pb.upgrade().unwrap().println(format!("[{}] {}", "Connection Error".red(), &url));
                            busy_threads_count_clone.fetch_sub(1, Ordering::SeqCst);
                            pb.upgrade().unwrap().inc(1);
                            continue;
                        }
                    };
                    
                    if resp_basic.status() != 404 {
                        // Move to start of line but do not erase any characters.
                        print!("\r");
                        if resp_basic.status() == 200 {
                            // Empty space needed to overwrite old characters written before printing "\r"
                            pb.upgrade().unwrap().println(format!("[ {} ]    {}                                                   ", resp_basic.status().as_str().green().bold(), &url));
                        } else {
                            pb.upgrade().unwrap().println(format!("[ {} ]    {}                                                   ", resp_basic.status().as_str().blue(), &url));
                        }
                        io::stdout().flush().unwrap();
                    };
                pb.upgrade().unwrap().inc(1);
                    
                // Decrement "number of busy threads" by 1
                busy_threads_count_clone.fetch_sub(1, Ordering::SeqCst);
            }
        }));
    }
    
    // Drop receiver for current thread
    recv.unsubscribe();
    
    // Iterate through each word in wordlist
    for word in wordlist  {
        thread::sleep(std::time::Duration::from_millis(user_configs.wait));

        print!("\r");
        print!("[=> ] Requesting - {}                                                     ", format!("{}{}", user_configs.url.clone(), &word));
        print!("\r");
        io::stdout().flush().unwrap();

        if word.chars().count() != 0 {
            if word.chars().nth(0).unwrap() == '/' {
                word.to_string().replace_range(1..word.chars().count(), "");
            }
        }
        // Infinite loop. Break if message aka URL is inserted to multiqueue.
        loop {
            if pass_urls(format!("{}{}", user_configs.url.clone(), word), busy_threads_count.load(Ordering::Relaxed), &send, user_configs.threads) {
                break;
            };
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }

    progress_bar.println("Wordlist Complete");
    progress_bar.inc(1);

    // Wait for other threads to finish processing elements in multiqueue
    loop {
        thread::sleep(std::time::Duration::from_millis(2000));
        if !progress_bar.is_finished() {
            break;
        }
    }

    println!("Droping Queue!");
    // Destroy multiqueue
    drop(send);
    println!("Closing Threads!");

    // Wait for threads to exit.
    for i in handle {
        i.join().unwrap();
    }
    println!("Exiting....");
}

// Send provided URL to provided channel if "number of busy threads" are less than "threads running"
fn pass_urls(url: String, busy_number: usize, send: &multiqueue::BroadcastSender<Box<String>>, thread_count: usize) -> bool {
    if busy_number <= thread_count {
        match send.try_send(Box::new(url)).is_ok() {
            true => {},
            false => {
                //println!("Queue Full");
                return false;
            },
        };
        true
    } else {
        // Sleep for 1 Second, if threads are busy
        thread::sleep(std::time::Duration::from_millis(1000));
        false
    }
}
