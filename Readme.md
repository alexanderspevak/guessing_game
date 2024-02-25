# Guessing Game

A client-server guessing game with a web interface for monitoring progress. Multiple clients can connect to the server to participate.

## Quick Start

```sh
# Start the server (replace <password> with your desired password)
cd server && cargo run -- --password=<password>

# In a new terminal, start a client
cd client && cargo run

# Repeat the client step for as many clients as needed

# Access the web interface at http://localhost:3000
