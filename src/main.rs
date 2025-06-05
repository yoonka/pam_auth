use pam::Authenticator;
use std::env;
use std::process;

fn main() {
    // Read username and password from command-line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: pam_auth <username> <password>");
        process::exit(1);
    }

    let username = &args[1];
    let password = &args[2];

    // Create a PAM Authenticator using the "login" service
    let mut auth = match Authenticator::with_password("login") {
        Ok(auth) => auth,
        Err(err) => {
            eprintln!("PAM setup failed: {}", err);
            process::exit(1);
        }
    };

    // Set credentials and authenticate
    auth.get_handler().set_credentials(username, password);

    match auth.authenticate() {
        Ok(()) => {
            println!("Authentication successful");
            process::exit(0);
        }
        Err(err) => {
            eprintln!("Authentication failed: {}", err);
            process::exit(2);
        }
    }
}
