import socket
import json
from threading import Thread
import smtplib
from email.mime.text import MIMEText
from email.mime.multipart import MIMEMultipart

def send_email_via_smtp(email_data):
    smtp_server = "127.0.0.1"  # IP dari PC3 (SMTP Server)
    smtp_port = 2525            # Port yang sama seperti yang digunakan di server SMTP PC3

    try:
        # Membuat pesan email
        msg = MIMEMultipart()
        msg['From'] = email_data['sender']
        msg['To'] = email_data['recipient']
        msg['Subject'] = email_data['subject']
        msg.attach(MIMEText(email_data['body'], 'plain'))

        # Mengirim email melalui SMTP (PC3)
        with smtplib.SMTP(smtp_server, smtp_port) as server:
            server.sendmail(email_data['sender'], email_data['recipient'], msg.as_string())

        # print("Email successfully sent via SMTP.")
        return "Email successfully sent via SMTP."
    except Exception as e:
        # print(f"Error sending email via SMTP: {e}")
        error_message = f"Error sending email via SMTP error message: {e}"
        return error_message

def handle_client(client_socket):
    try:
        data = client_socket.recv(1024).decode()
        email_data = json.loads(data.replace("'", "\""))  # Parse the string as JSON
        print(f"Received email data: {email_data}")
        result = send_email_via_smtp(email_data)  # Kirim email via SMTP (ke PC3)
        print(f"iki lo hasil e {result.encode()}")
        client_socket.sendall(result.encode())

    except Exception as e:
        print(f"Error handling client: {e} {result}")
        client_socket.sendall(b"Failed to forward email".encode())
    finally:
        client_socket.close()

def start_forwarder(server_ip, server_port):
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as server_socket:
        server_socket.bind((server_ip, server_port))
        server_socket.listen(5)
        print(f"Forwarder listening on {server_ip}:{server_port}...")
        while True:
            client_socket, addr = server_socket.accept()
            print(f"Connection established with {addr}")
            thread = Thread(target=handle_client, args=(client_socket,))
            thread.start()

if __name__ == "__main__":
    SERVER_IP = "127.0.0.1"
    SERVER_PORT = 1287
    start_forwarder(SERVER_IP, SERVER_PORT)
