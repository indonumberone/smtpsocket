use serde::Deserialize;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Deserialize, Debug)]
struct Email {
    sender: String,
    recipient: String,
    subject: String,
    body: String,
}

/// Mengirim email ke server SMTP
async fn send_email_via_smtp(email: Email) -> Result<String> {
    let stream = TcpStream::connect("127.0.0.1:25").await?;
    let mut stream = BufReader::new(stream);

    // Kirim EHLO
    stream.write_all(b"EHLO localhost\r\n").await?;
    let mut response = vec![0; 1024];
    stream.read(&mut response).await?;
    println!(
        "SMTP Response (EHLO): {}",
        String::from_utf8_lossy(&response)
    );

    // Kirim MAIL FROM
    let mail_from = format!("MAIL FROM:<{}>\r\n", email.sender);
    stream.write_all(mail_from.as_bytes()).await?;
    response.clear();
    stream.read(&mut response).await?;
    println!(
        "SMTP Response (MAIL FROM): {}",
        String::from_utf8_lossy(&response)
    );

    // Kirim RCPT TO
    let rcpt_to = format!("RCPT TO:<{}>\r\n", email.recipient);
    stream.write_all(rcpt_to.as_bytes()).await?;
    response.clear();
    stream.read(&mut response).await?;
    println!(
        "SMTP Response (RCPT TO): {}",
        String::from_utf8_lossy(&response)
    );

    // Kirim DATA
    stream.write_all(b"DATA\r\n").await?;
    response.clear();
    stream.read(&mut response).await?;
    println!(
        "SMTP Response (DATA): {}",
        String::from_utf8_lossy(&response)
    );

    // Kirim isi email
    let email_body = format!("Subject: {}\r\n\r\n{}\r\n.\r\n", email.subject, email.body);
    stream.write_all(email_body.as_bytes()).await?;
    response.clear();
    stream.read(&mut response).await?;
    println!(
        "SMTP Response (email body): {}",
        String::from_utf8_lossy(&response)
    );

    // Kirim QUIT
    stream.write_all(b"QUIT\r\n").await?;
    response.clear();
    stream.read(&mut response).await?;
    println!(
        "SMTP Response (QUIT): {}",
        String::from_utf8_lossy(&response)
    );

    Ok("Email successfully sent via SMTP.".to_string())
}

/// Menangani client yang terhubung ke forwarder
async fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 4096];
    match stream.read(&mut buffer).await {
        Ok(size) => {
            let received_data = String::from_utf8_lossy(&buffer[..size]);
            println!("Received email data: {}", received_data);

            match serde_json::from_str::<Email>(&received_data) {
                Ok(email) => {
                    let result =
                        tokio::spawn(async move { send_email_via_smtp(email).await }).await;

                    match result {
                        Ok(Ok(response)) => {
                            println!("Email sent: {}", response);
                            let _ = stream.write_all(response.as_bytes()).await;
                        }
                        Ok(Err(error)) => {
                            println!("Error: {}", error);
                            let _ = stream.write_all(error.to_string().as_bytes()).await;
                        }
                        Err(join_error) => {
                            println!("Task failed to run: {}", join_error);
                            let _ = stream.write_all(b"Internal server error").await;
                        }
                    }
                }
                Err(e) => {
                    println!("Error parsing email data: {}", e);
                    let _ = stream.write_all(b"Invalid email data").await;
                }
            }
        }
        Err(e) => {
            println!("Error reading from client: {}", e);
            let _ = stream.write_all(b"Failed to read data").await;
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:1239").await?;
    println!("Listening on 127.0.0.1:1234...");

    loop {
        let (stream, addr) = listener.accept().await?;
        println!("Connection from {:?}", addr);

        tokio::spawn(async move {
            handle_client(stream).await;
        });
    }
}
