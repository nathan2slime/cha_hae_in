
# Cha Hae-In
> **Note**: This file is modified by a LLM trained with the personality of an anime girl, ready to make everything more kawaii! 💖

Rust-based Discord bot using the serenity library, inspired by Cha Hae-In, the amazing S-Rank Hunter from Solo Leveling! 🗡️
Cha Hae-In is strong 💪, graceful 💖, and always loyal to her friends. She's a true heroine who uses her mighty sword to protect those she loves. 🌟

##🎮 Commands

> **Note**: All commands are super kawaii **slash commands** (`/command`). 

Here’s what this bot can do for you:  

1. **Ping**  
   - Command: `/ping`  
   - Checks how fast I can respond to you! Zoom zoom~ 

2. **Clear Messages**  
   - Command: `/clear`  
   - Tidies up messy channels. Let’s keep things clean, ok?   

3. **Astronomy Picture of the Day**  
   - Command: `/nasa apod`  
   - Wanna see the stars? Let’s check out NASA’s picture of the day!  

4. **Random Anime**  
   - Command: `/anime random`  
   - Need a new anime to watch? I’ll suggest something for you!   

5. **Music Commands** 
   - `/music play [url]`: Plays music from the URL (e.g., YouTube). Let’s vibe together!
   - `/music join_channel`: Joins the voice channel you’re in! Let’s hang out!

---

## Setup
### 1. Install Rust
If you don't have Rust yet, don't worry! 🥺 You can easily download and install it from the official Rust website. 🌟

### 4. Install Cargo (if you don't have it)
If you don't have Cargo yet, don't worry! 😇 It's super easy to install! Just run this cute command:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Create the `.env` File
In the root of the project, create a `.env` file with the following content:

```.env
# Discord bot token
DISCORD_TOKEN=""

# Logging level for Rust application
RUST_LOG="info"

# Database connection URL
DATABASE_URL="sqlite::memory:chahaein"

# NASA API key for accessing NASA data
NASA_API_KEY=""
```

Replace your_token_here with your actual Discord bot token. 💖✨

### 3. Install Dependencies and Run
In the terminal, navigate to the project folder and run the command to install dependencies and get Cha Hae-In up and running:

```bash
cargo run
```

## 🐳 Using Docker

### Setup
To run the bot in production using Docker Compose:

### Install docker and docker-compose.
Create a `.env.production` file as described above.

### Running
To start the bot:

```bash
docker-compose up
```

Yay!, everything is set up for you to start having fun with the bot! 🌸💕
