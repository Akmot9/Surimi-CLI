use std::io;
use file_integrity::{list_files, hash_file_list, write_json_file} ;
use colored::Colorize;

fn main() {
    title();

    let estimate_time = ask_yes_no("Do you want to estimate the time of the scan?");
    if estimate_time {
        println!("Estimating time...");
        let folder_path = "/"; // Change this to the desired folder path
        let nbs_of_file = list_files(&folder_path);
        println!("INFOS: Number of files: {}", nbs_of_file);

        let generate_report = ask_yes_no("Do you want to generate the integrity report?");
        if generate_report {
            let hashs = hash_file_list();
            let name = "output.json";
            write_json_file(&hashs, name) ;
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