# Implementation of Email Forwarder with SMTP Server in Rust

[![forthebadge](http://forthebadge.com/images/badges/made-with-rust.svg)](http://forthebadge.com)
[![forthebadge](http://forthebadge.com/images/badges/built-with-love.svg)](http://forthebadge.com)

[![Crate Version](https://img.shields.io/crates/v/colorls.svg)](https://crates.io/)

[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=shields)](http://makeapullrequest.com)

SMTP (Simple Mail Transfer Protocol) is a protocol used to send emails over the internet. In this project, the email will first be forwarded through a forwarder built using the Python programming language. Then, the email will be sent to an SMTP server implemented with the Rust programming language, in accordance with the standards outlined in RFC 5321. This project aims to fulfill the final semester assignment for the Computer Systems and Networks course.

## Table of Contents

- [Overview](#overview)
- [Requirements](#requirements)
- [Installation](#installation)
- [Usage](#usage)
- [Configuration](#configuration)
- [License](#license)

## Overview

This system consists of four main components:

1. **Email Forwarder (Python)**: A Python-based service that forwards email data received from clients to the SMTP server.
2. **SMTP Server (Rust)**: A server written in Rust that receives forwarded emails and sends them to the recipient.
3. **HTTP Server (Node.js)**: An optional HTTP server that can be used to send email data via HTTP requests.
4. **Email Client (Rust)**: A simple command-line client written in Rust that sends email data to the Python email forwarder.

## Requirements

- **Python 3.x** (for Email Forwarder)
- **Rust** (for SMTP Server and Email Client)
- **Node.js** (optional, for HTTP server)
- **Libraries**:
  - `serde` and `serde_json` for serializing email data in Rust.
  - `tokio` for asynchronous operations in Rust.
- **Ports**:
  - SMTP server listens on port `2525`.
  - Email forwarder listens on port `1239`.
  - HTTP server (if used) listens on port `8080`.

## Installation

### Python Email Forwarder

1. Clone the repository:

   ```bash
   cd forwarder
   ```

2. Install the required Python dependencies:

   Although `smtplib`, `email`, and `json` are part of the Python standard library, you can create a virtual environment to keep your project isolated.

   ```bash
   python3 -m venv venv
   source venv/bin/activate  # On Windows, use `venv\Scripts\activate`
   ```

   Install any additional dependencies if needed:

   ```bash
   pip install -r requirements.txt
   ```

### Rust SMTP Server

1. Ensure you have **Rust** installed. If not, follow the installation instructions here: [Install Rust](https://www.rust-lang.org/learn/get-started).
2. Clone the SMTP server repository and build the server using Cargo:

   ```bash
   cd server
   cargo build --release
   ```

3. The server will listen on port `2525` by default. You can change this configuration in the Rust server code if needed.

### Node.js HTTP Server

1. Install **Node.js**. You can download it from [nodejs.org](https://nodejs.org/).

2. Navigate to the directory where the HTTP server files are located and install the dependencies:

   ```bash
   cd web
   npm install
   ```

3. The HTTP server will be used to send email requests via HTTP.

### Email Client (Rust)

1. Create a new **Rust** project (if you haven't already):

   ```bash
   cd socket
   cargo build --release
   ```

The client will prompt you to input the recipient, subject, and body of the email. Once the email data is entered, it will be serialized to JSON and sent to the **Email Forwarder** (Python-based service), which will forward the email to the **Rust SMTP server**.

## Usage

### Running the SMTP Server

```bash
cargo run --release
```

The smtp server will listen on port `2525`

### Running the Email Forwarder (Python)

Run the Python-based email forwarder:

```bash
python3 forwarder.py
```

The forwarder will listen on port `1239` for incoming email data. It forwards the email data to the Rust-based SMTP server, which will process it.

### Sending Email via Rust Client

```bash
cargo run --release

```

Enter the recipient, subject, and body of the email when prompted.

### HTTP Server

You can use a Node.js-based HTTP server to display or view emails that have already been received and stored.

Start the HTTP server:

```bash
npm start
```

## Configuration

You can configure the following:

- **SMTP Server Configuration**: Modify the Rust SMTP server to change the port or add other configuration options.
- **Email Forwarder**: Configure the Python script to change the SMTP server or its behavior.
- **Client Configuration**: You can modify the Rust client to include additional email fields or change the way emails are sent.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.
