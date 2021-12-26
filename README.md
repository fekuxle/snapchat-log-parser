![license](https://img.shields.io/crates/l/snapchat-log-parser) 
![crates.io version](https://img.shields.io/crates/v/snapchat-log-parser)

# Snapchat log parser

A simple crate to parse Snapchat chat logs

# Installation
Add the following to your `Cargo.toml` file:

```toml
[dependencies]
snapchat_log_parser = "0.3"
```

# How to get your chat logs

1. Log in to your account on accounts.snapchat.com
2. Click **“My data”**.
3. Click **“Submit request”** at the bottom of the page.
4. Wait for Snapchat to send you your data
5. When your data arrives, go to `data/json/chat_histoy.json`.
This contains all your sent and recieved chats