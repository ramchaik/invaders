use std::error::Error;
use rusty_audio::Audio;

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode", "sounds/explode.wav");
    audio.add("lose", "sounds/lose.wav");
    audio.add("move", "sounds/move.wav");
    audio.add("startup", "sounds/startup.wav");
    audio.add("pew", "sounds/pew.wav");
    audio.add("win", "sounds/win.wav");
    audio.play("startup");

    // Cleanup
    audio.wait();
    Ok(())
}
