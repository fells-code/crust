use crossterm::{
    event::{read, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use std::io::{self, stdout, Write, Read};
use std::process::{Command, Stdio};


fn run_command(input: &str) {
    let mut parts = input.trim().split_whitespace();
    
    if let Some(cmd) = parts.next() {
        let args: Vec<&str> = parts.collect();

        // Exit raw mode so output displays normally
        let _ = disable_raw_mode();

        let child_result = Command::new(cmd)
            .args(&args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn();

        match child_result {
            Ok(mut child) => {
                if let Some(mut out) = child.stdout.take() {
                    let mut stdout_buf = String::new();
                    out.read_to_string(&mut stdout_buf).unwrap_or(0);
                    print!("{stdout_buf}");
                }

                if let Some(mut err) = child.stderr.take() {
                    let mut stderr_buf = String::new();
                    err.read_to_string(&mut stderr_buf).unwrap_or(0);
                    eprint!("{stderr_buf}");
                }

                let _ = child.wait();
            }
            Err(e) => {
                eprintln!("⚠️ Error running command: {e}");
            }
        }

        // Re-enable raw mode for Crust
        let _ = enable_raw_mode();
    }
}

fn main() ->io::Result<()> {
    let mut stdout = stdout();

    enable_raw_mode()?;
    execute!(stdout, Clear(ClearType::All))?;

    println!("Welcome to Crust 🦀\r");
    println!("Type a command (ESC to exit)\r");
    print!("> ");

    let mut buffer = String::new();

    loop {
        match read()? {
            Event::Key(event) => {
                match event.code {
                    KeyCode::Esc => break,
                    KeyCode::Enter => {
                        println!("\r");
                        run_command(&buffer);
                        buffer.clear();
                        print!("\r");
                        print!("> ");
                        stdout.flush()?;
                    }
                    KeyCode::Char(c) => {
                        buffer.push(c);
                        print!("{c}");
                        stdout.flush()?;
                    }
                    KeyCode::Backspace => {
                        if buffer.pop().is_some() {
                            print!("\x08 \x08");
                            stdout.flush()?;
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    disable_raw_mode()?;
    Ok(())
}
