from flask import Flask, request, render_template
import json

app = Flask(__name__)

def load_received_emails():
    try:
        with open("received_emails.json", "r") as f:
            return json.load(f)
    except FileNotFoundError:
        return []  # Jika file tidak ditemukan, kembalikan list kosong

@app.route("/")
def index():
    email_messages = load_received_emails()  # Muat email yang disimpan dari file
    return render_template("index.html", messages=email_messages)

if __name__ == "__main__":
    app.run(host="127.0.0.1", port=5000)
