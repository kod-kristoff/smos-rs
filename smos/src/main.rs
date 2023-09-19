mod app;
mod components;

use tuirealm::{AttrValue, Attribute, PollStrategy};

use crate::app::{Id, PicoSmosApp};

fn main() {
    println!("Hello, world!");
    let app = PicoSmosApp::default();

    // Enter alternate screen
    let _ = app.terminal.enter_alternate_screen();
    let _ = app.terminal.enable_raw_mode();
    // Main loop
    // NOTE: loop until quit; quit is set in update if AppClose is received from counter
    while !app.quit {
        // Tick
        match app.app.tick(PollStrategy::Once) {
            Err(err) => {
                assert!(app
                    .app
                    .attr(
                        &Id::Label,
                        Attribute::Text,
                        AttrValue::String(format!("Application error: {}", err)),
                    )
                    .is_ok());
            }
            Ok(messages) if messages.len() > 0 => {
                // NOTE: redraw if at least one msg has been processed
                app.redraw = true;
                for msg in messages.into_iter() {
                    let mut msg = Some(msg);
                    while msg.is_some() {
                        msg = app.update(msg);
                    }
                }
            }
            _ => {}
        }
        // Redraw
        if app.redraw {
            app.view();
            app.redraw = false;
        }
    }
    // Terminate terminal
    let _ = app.terminal.leave_alternate_screen();
    let _ = app.terminal.disable_raw_mode();
    let _ = app.terminal.clear_screen();
}
