# Discord Music Bot  
A Discord music bot written in Rust.

## Prerequisites  

1. **Rust Tools**  
You can install Rust from the official website: [Rust installation page](https://www.rust-lang.org/tools/install)

2. **FFmpeg**  
Download FFmpeg from [FFmpeg's website](https://ffmpeg.org/) and ensure it is available in your PATH.

3. **yt-dlp**  
You can download it here: [yt-dlp](https://github.com/yt-dlp/yt-dlp). Make sure yt-dlp is also available in your PATH.

4. **Create `config.json`**  
Create config.json and insert prefix and discord app token (example_config.json).

## Usage  

1. **Start the Bot**  
To run the bot, execute the following command:  
```sh
cargo run --release
```

2. **Deploy Slash Commands**  
Before using slash commands, run the deploy command to register them on your server:  
```sh
<prefix from config>deploy
```
Replace `<prefix from config>` with the prefix defined in your config.json file.