import asyncio
from email.parser import BytesParser
from email.policy import default
from aiosmtpd.controller import Controller
import json

# Tempat penyimpanan email yang diterima
received_emails = []

class CustomSMTPHandler:
    async def handle_DATA(self, server, session, envelope):
        try:
            # Parse email
            message = BytesParser(policy=default).parsebytes(envelope.content)
            email_data = {
                "sender": envelope.mail_from,
                "recipient": envelope.rcpt_tos,
                "subject": message['subject'],
                "body": message.get_body(preferencelist=('plain')).get_payload(decode=True).decode()
            }

            # Simpan email ke daftar
            received_emails.append(email_data)
            print(f"Received email from {envelope.mail_from} to {envelope.rcpt_tos}")
            print(f"Subject: {message['subject']}")

            # Simpan email ke file (misalnya email.json)
            with open("received_emails.json", "w") as f:
                json.dump(received_emails, f, indent=4)

            return '250 OK'
        except Exception as e:
            print(f"Error processing email: {e}")
            return '550 Failed'

def start_smtp_server():
    handler = CustomSMTPHandler()
    controller = Controller(handler, hostname='0.0.0.0', port=2525)  # Host dan port SMTP server
    controller.start()
    print("SMTP server started on 0.0.0.0:2525")
    try:
        # Keep the server running
        asyncio.get_event_loop().run_forever()
    except KeyboardInterrupt:
        pass
    finally:
        controller.stop()

if __name__ == "__main__":
    start_smtp_server()
