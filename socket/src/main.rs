use serde::Serialize;
use std::io::{self, Read, Write};
use std::net::TcpStream;

#[derive(Serialize)]
struct Email {
    sender: String,
    recipient: String,
    subject: String,
    body: String,
}

fn send_message(email: &str) {
    let server = "127.0.0.1:1239";

    match TcpStream::connect(server) {
        Ok(mut stream) => {
            println!("Succsessfully connected to forwarder {}", server);
            match stream.write_all(email.as_bytes()) {
                Ok(_) => {
                    println!("Try to sent email to forwarder");
                }
                Err(e) => {
                    println!("Failed to send email to forwarder {}", e);
                }
            }
            let mut response = String::new();
            match stream.read_to_string(&mut response) {
                Ok(_) => println!("Server response: {}", response),
                Err(e) => eprintln!("Failed to read server response: {}", e),
            }
        }
        Err(e) => {
            println!("Failed to connect to forwarder {}", e)
        }
    }
}

fn main() {
    let mut sender = "ketua@kelompoksatu.com";
    let mut recipient = String::new();
    let mut subject = String::new();
    let mut body = String::new();

    println!("=== Email Client ===");

    loop {
        print!("To: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut recipient).unwrap();
        recipient = recipient.trim().to_string();
        print!("Subject: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut subject).unwrap();
        subject = subject.trim().to_string();
        print!("body: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut body).unwrap();
        body = body.trim().to_string();

        let email = Email {
            sender: sender.to_string(),
            recipient: recipient.clone(),
            subject: subject.clone(),
            body: body.clone(),
        };

        let email_json = serde_json::to_string(&email).unwrap();
        // println!("emaile: {}", email_json);
        send_message(&email_json);
        let mut choose = String::new();
        print!("NGIRIM MENEH??? y/n: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut choose).unwrap();
        choose = choose.trim().to_string();
        if choose.eq_ignore_ascii_case("n") {
            break;
        }
        recipient.clear();
        body.clear();
    }
    println!("Seeee yooooouuuuuuuuuu");
}
