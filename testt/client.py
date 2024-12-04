import socket
import json

def client_interface(forwarder_ip, forwarder_port):
    print("=== Email Client ===")
    while True:
        recipient = input("To: ")
        subject = input("Subject: ")
        body = input("Body:\n")
        
        email_message = {
            "sender": "ketua@group1.com",
            "recipient": recipient,
            "subject": subject,
            "body": body
        }
        
        send_message(forwarder_ip, forwarder_port, email_message)
        if input("\nSend another email? (y/n): ").lower() != "y":
            break

def send_message(forwarder_ip, forwarder_port, email_message):
    try:
        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as client_socket:
            client_socket.connect((forwarder_ip, forwarder_port))
            client_socket.sendall(json.dumps(email_message).encode())
            response = client_socket.recv(1024).decode()
            print(f"Server response: {response}")
    except Exception as e:
        print(f"Error sending message: {e}")

if __name__ == "__main__":
    FORWARDER_IP = "127.0.0.1"  # Localhost IP for testing
    FORWARDER_PORT = 1239    # Port used by Forwarder
    client_interface(FORWARDER_IP, FORWARDER_PORT)
