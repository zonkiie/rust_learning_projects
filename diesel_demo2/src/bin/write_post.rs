use diesel_demo2::*;
use std::io::*;

fn main() {
    let connection = establish_connection();

    println!("What would you like your title to be?");
    let mut title = String::new();
    stdin().read_line(&mut title).unwrap();
    let title = &title[..(title.len() - 1)]; // Drop the newline character
    
    let mut owner_id = String::new();
    println!("Your User-ID:");
	stdin().read_line(&mut owner_id)
        .ok()
        .expect("Failed to read line");
    let owner_id: i32 = owner_id.trim().parse()
		.ok()
		.expect("Please type a number!");
    
    println!(
        "\nOk! Let's write {} (Press {} when finished)\n",
        title, EOF
    );
    let mut body = String::new();
    stdin().read_to_string(&mut body).unwrap();

    let _ = create_post(&connection, title, &body, &owner_id);
    println!("\nSaved draft {}", title);
}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";
