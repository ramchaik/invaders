use invaders::{
    frame::{self, Drawable},
    render, player::Player,
};
use std::{error::Error, io, time::Duration, sync::mpsc, thread};
use crossterm::{terminal::{self, EnterAlternateScreen, LeaveAlternateScreen}, cursor::{Hide, Show}, ExecutableCommand, event::{self, KeyCode}};
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

    // Terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // Render loop in separate thread
    let (render_tx, render_rx) = mpsc::channel();
    let render_thread = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let curr_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };
            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });
    

    // Gameloop
    let mut player = Player::new();
    'gameloop: loop {
        // Per frame init
        let mut curr_frame = frame::new_frame();

        // Input
        while event::poll(Duration::default())? {
            if let event::Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    },
                    _ => {}
                }
            }
        }

        // Draw & Render
        player.draw(&mut curr_frame);
        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1));
    }
    
    // Cleanup
    drop(render_tx);
    render_thread.join().unwrap();
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
