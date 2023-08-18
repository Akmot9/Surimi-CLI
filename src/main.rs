use std::{io, sync::Arc};
//use file_integrity::{list_files, hash_file_list, write_json_file} ;
use std::{thread, time::Duration, io::Write};
use colored::Colorize;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::file_integrity::{hash_file_list, list_files, write_json_file};

mod file_integrity ;

const SPINNER_FRAMES: [&str; 10] = [
    "🦀        ",
    "🦀🔪     ",
    "🦀  🔪   ",
    "🦀🔪     ",
    "🦀        ",
    "🦀🔪     ",
    "🦀  🔪   ",
    "🦀🔪     ",
    "🍥🔪     ",
    "🍥       ",
];


fn main() {
    title();

    let estimate_time = ask_yes_no("Do you want to estimate the time of the scan?");
    if estimate_time {
        println!("Estimating time...");
        let folder_path = "/";

        // Use an AtomicBool flag to communicate with the spinner thread
        let should_stop_spinner = Arc::new(AtomicBool::new(false));

        // Clone the AtomicBool flag for the closure
        let should_stop_spinner_clone = Arc::clone(&should_stop_spinner);

        // Spawn a new thread to run the spinner function concurrently with the list_files function
        let spinner_thread = thread::spawn(move || run_spinner(&should_stop_spinner_clone));

        let nbs_of_file = list_files(&folder_path);

        // Signal the spinner thread to stop by setting the AtomicBool flag
        should_stop_spinner.store(true, Ordering::SeqCst);

        // Wait for the spinner thread to finish
        spinner_thread.join().unwrap();

        pre_calcule(nbs_of_file) ;

        let generate_report = 
        ask_yes_no("Do you want to generate the integrity report ?");
        if generate_report {
            let name = ask_for_json_filename();
            println!("Integrity reports in progress...");
            let hashs = hash_file_list();
            
            write_json_file(&hashs, &name) ;
            println!("JSON report written successfully.");
        } else {
            println!("Report generation cancelled.");
        }
    } else {
        println!("No time estimation requested.");
    }
}
fn title() {
    let text = "
  _________            .__        .__ 
 /   _____/__ _________|__| _____ |__|
 \\_____  \\|  |  \\_  __ \\  |/     \\|  |
 /        \\  |  /|  | \\/  |  Y Y  \\  |
/_______  /____/ |__|  |__|__|_|  /__|
        \\/                      \\/    

A simple cli app to make integrity reports of your computer.\n";
    println!("{}", text.truecolor(239, 112, 90));
}

fn ask_yes_no(question: &str) -> bool {
    loop {
        println!("{} (y/n)", question);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().to_lowercase().as_str() {
            "y" => return true,
            "n" => return false,
            _ => println!("Please enter 'y' or 'n'."),
        }
    }
}

fn run_spinner(should_stop_spinner: &AtomicBool) {
    // Hide the cursor.
    print!("\x1B[?25l");
    io::stdout().flush().unwrap();

    let mut frame_index = 0;
    while !should_stop_spinner.load(Ordering::SeqCst) {
        print!("\r\x1B[K{}", SPINNER_FRAMES[frame_index]);
        io::stdout().flush().unwrap();

        frame_index = (frame_index + 1) % SPINNER_FRAMES.len();

        thread::sleep(Duration::from_millis(200));
    }

    // Show the cursor again.
    print!("\x1B[?25h");
    io::stdout().flush().unwrap();
}

fn pre_calcule(nb: i32) {
    println!("Number of files to hash: {}", nb);
}

fn ask_for_json_filename() -> String {
    loop {
        println!("Please enter the name of the JSON file (or press Enter to cancel):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let name = input.trim().to_string();
        if !name.is_empty() {
            return name;
        } else {
            println!("Invalid entry. Please provide a non-empty filename.");
        }
    }
}