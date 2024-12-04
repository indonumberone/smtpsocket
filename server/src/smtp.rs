// use crate::database;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fs::File;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Mail {
    pub from: String,
    pub to: Vec<String>,
    pub data: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum State {
    Fresh,
    Greeted,
    ReceivingRcpt(Mail),
    ReceivingData(Mail),
    Received(Mail),
}

struct StateMachine {
    state: State,
    ehlo_greeting: String,
}

impl StateMachine {
    const OH_HAI: &'static [u8] = b"220 kelompok1\n";
    const KK: &'static [u8] = b"250 Ok\n";
    const AUTH_OK: &'static [u8] = b"235 Ok\n";
    const SEND_DATA_PLZ: &'static [u8] = b"354 End data with <CR><LF>.<CR><LF>\n";
    const KTHXBYE: &'static [u8] = b"221 Bye\n";
    const HOLD_YOUR_HORSES: &'static [u8] = &[];

    pub fn new(domain: impl AsRef<str>) -> Self {
        let domain = domain.as_ref();
        let ehlo_greeting = format!("250-{domain} Hello {domain}\n250 AUTH PLAIN LOGIN\n");
        Self {
            state: State::Fresh,
            ehlo_greeting,
        }
    }

    pub fn handle_smtp(&mut self, raw_msg: &str) -> Result<&[u8]> {
        let mut msg = raw_msg.split_whitespace();
        let command = msg.next().context("received empty command")?.to_lowercase();
        let state = std::mem::replace(&mut self.state, State::Fresh);
        match (command.as_str(), state) {
            ("ehlo", State::Fresh) => {
                self.state = State::Greeted;
                Ok(self.ehlo_greeting.as_bytes())
            }
            ("helo", State::Fresh) => {
                self.state = State::Greeted;
                Ok(StateMachine::KK)
            }
            ("mail", State::Greeted) => {
                let from = msg.next().context("received empty MAIL")?;
                let from = from
                    .strip_prefix("FROM:")
                    .context("received incorrect MAIL")?;
                self.state = State::ReceivingRcpt(Mail {
                    from: from.to_string(),
                    ..Default::default()
                });
                Ok(StateMachine::KK)
            }
            ("rcpt", State::ReceivingRcpt(mut mail)) => {
                let to = msg.next().context("received empty RCPT")?;
                let to = to.strip_prefix("TO:").context("received incorrect RCPT")?;
                let to = to.to_lowercase();
                if Self::legal_recipient(&to) {
                    mail.to.push(to);
                }
                self.state = State::ReceivingRcpt(mail);
                Ok(StateMachine::KK)
            }
            ("data", State::ReceivingRcpt(mail)) => {
                self.state = State::ReceivingData(mail);
                Ok(StateMachine::SEND_DATA_PLZ)
            }

            ("quit", State::ReceivingData(mail)) => {
                self.state = State::Received(mail);
                Ok(StateMachine::KTHXBYE)
            }
            (_, State::ReceivingData(mut mail)) => {
                let resp = if raw_msg.ends_with("\r\n.\r\n") {
                    StateMachine::KK
                } else {
                    StateMachine::HOLD_YOUR_HORSES
                };
                mail.data += raw_msg;
                self.state = State::ReceivingData(mail);
                Ok(resp)
            }
            ("rset", _) => {
                self.state = State::Greeted; // Reset state
                Ok(StateMachine::KK)
            }
            ("noop", _) => Ok(StateMachine::KK),
            ("vrfy", _) => Ok(b"252 Cannot VRFY user, but will accept message\n"),
            ("expn", _) => Ok(b"252 Mailing lists are not supported\n"),
            (_, _) => {
                // Return a 500 error for unrecognized commands
                Ok(b"500 Syntax error, command unrecognized\n")
            }
        }
    }

    fn legal_recipient(to: &str) -> bool {
        !to.contains("admin") && !to.contains("postmaster") && !to.contains("hostmaster")
    }
}

pub struct Server {
    stream: tokio::net::TcpStream,
    state_machine: StateMachine,
    // db: Arc<Mutex<database::Client>>,
}

impl Server {
    pub async fn new(domain: impl AsRef<str>, stream: tokio::net::TcpStream) -> Result<Self> {
        Ok(Self {
            stream,
            state_machine: StateMachine::new(domain),
        })
    }

    pub async fn serve(mut self) -> Result<()> {
        self.greet().await?;

        let mut buf = vec![0; 1024 * 1024];
        loop {
            let n = self.stream.read(&mut buf).await?;

            if n == 0 {
                self.state_machine.handle_smtp("quit").ok();
                break;
            }
            let msg = std::str::from_utf8(&buf[0..n])?;
            let response = self.state_machine.handle_smtp(msg)?;
            if response != StateMachine::HOLD_YOUR_HORSES {
                self.stream.write_all(response).await?;
            }
            if response == StateMachine::KTHXBYE {
                break;
            }
        }

        match self.state_machine.state {
            State::Received(ref mail) => {
                // Save email to file as JSON
                self.save_email_to_json(mail).await?;
                // self.db.lock().await.replicate(mail.clone()).await?;
            }
            State::ReceivingData(ref mail) => {
                self.save_email_to_json(mail).await?;
                // self.db.lock().await.replicate(mail.clone()).await?;
            }
            _ => {}
        }
        Ok(())
    }

    async fn save_email_to_json(&self, mail: &Mail) -> Result<()> {
        let file_path = "received_email.json";
        let email_entry = json!({
            "sender": mail.from,
            "recipient": mail.to,
            "subject": self.extract_subject(&mail.data)?,
            "body": self.extract_body(&mail.data)?,
        });

        // Baca file JSON jika ada
        let mut emails: Vec<Value> = if let Ok(existing_content) = fs::read_to_string(file_path) {
            serde_json::from_str(&existing_content)?
        } else {
            Vec::new()
        };

        // Tambahkan email baru ke array
        emails.push(email_entry);

        // Tulis kembali ke file JSON
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_path)?;
        serde_json::to_writer_pretty(file, &emails)?;
        println!("Email saved to {}", file_path);

        Ok(())
    }

    fn extract_subject(&self, data: &str) -> Result<String> {
        for line in data.lines() {
            if line.to_lowercase().starts_with("subject:") {
                return Ok(line[8..].trim().to_string());
            }
        }
        Ok("No Subject".to_string())
    }

    fn extract_body(&self, data: &str) -> Result<String> {
        if let Some(boundary_start) = data.find("--===") {
            let body_start = data[boundary_start..]
                .find("\r\n\r\n")
                .map(|pos| boundary_start + pos + 4)
                .unwrap_or(data.len());
            let body_end = data[body_start..]
                .find("\r\n--===")
                .map(|pos| body_start + pos)
                .unwrap_or(data.len());
            return Ok(data[body_start..body_end].trim().to_string());
        }

        // Jika bukan multipart, cari bagian setelah header
        if let Some(headers_end) = data.find("\r\n\r\n") {
            return Ok(data[headers_end + 4..].trim().to_string());
        }

        Ok("No Body Found".to_string())
    }

    async fn greet(&mut self) -> Result<()> {
        self.stream
            .write_all(StateMachine::OH_HAI)
            .await
            .map_err(|e| e.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regular_flow() {
        let mut sm = StateMachine::new("dummy");
        sm.handle_smtp("HELO localhost").unwrap();
        sm.handle_smtp("MAIL FROM: <local@example.com>").unwrap();
        sm.handle_smtp("RCPT TO: <a@localhost.com>").unwrap();
        sm.handle_smtp("RCPT TO: <b@localhost.com>").unwrap();
        sm.handle_smtp("DATA hello world\n").unwrap();
        sm.handle_smtp("QUIT").unwrap();
    }

    #[test]
    fn test_no_greeting() {
        let mut sm = StateMachine::new("dummy");
        for command in [
            "MAIL FROM: <local@example.com>",
            "RCPT TO: <local@example.com>",
            "DATA hey",
            "GARBAGE",
        ] {
            assert!(sm.handle_smtp(command).is_err());
        }
    }
}
