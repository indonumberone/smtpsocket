import smtplib
from email.mime.text import MIMEText
from email.mime.multipart import MIMEMultipart

def send_email():
    # Informasi email
    from_email = "sender@example.com"
    to_email = "recipient@example.com"
    subject = "Hello from Python!"
    body = "This is a test email sent from Python."

    # Membuat objek MIME
    msg = MIMEMultipart()
    msg['From'] = from_email
    msg['To'] = to_email
    msg['Subject'] = subject

    # Menambahkan badan pesan (body)
    msg.attach(MIMEText(body, 'plain'))

    # Konfigurasi server SMTP lokal
    smtp_server = "localhost"  # Server SMTP lokal
    smtp_port = 2525             # Biasanya port SMTP lokal adalah 25

    try:
        # Menghubungkan ke server SMTP lokal
        with smtplib.SMTP(smtp_server, smtp_port) as server:
            # Mengirim email
            server.sendmail(from_email, to_email, msg.as_string())
            print("Email sent successfully!")
    except Exception as e:
        print(f"Error sending email: {e}")

if __name__ == "__main__":
    send_email()
