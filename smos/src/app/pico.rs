use std::time::Duration;

use tuirealm::{
    props::{Alignment, Color, TextModifiers},
    terminal::TerminalBridge,
    tui::widgets::canvas::Label,
    Application, EventListenerCfg, NoUserEvent, Update,
};

use super::{Id, Msg};

pub struct PicoSmosApp {
    pub app: Application<Id, Msg, NoUserEvent>,
    pub quit: bool,
    pub redraw: bool,
    pub terminal: TerminalBridge,
}

impl Default for PicoSmosApp {
    fn default() -> Self {
        Self {
            app: Self::init_app(),
            quit: false,
            redraw: true,
            terminal: TerminalBridge::new().expect("initialized terminal"),
        }
    }
}

impl PicoSmosApp {
    fn init_app() -> Application<Id, Msg, NoUserEvent> {
        let mut app: Application<Id, Msg, NoUserEvent> = Application::init(
            EventListenerCfg::default()
                .default_input_listener(Duration::from_millis(20))
                .poll_timeout(Duration::from_millis(10))
                .tick_interval(Duration::from_secs(1)),
        );

        // Mount components
        assert!(app
            .mount(
                Id::Label,
                Box::new(
                    Label::default()
                        .text("Waiting for a Msg...")
                        .alignment(Alignment::Left)
                        .background(Color::Reset)
                        .foreground(Color::LightYellow)
                        .modifiers(TextModifiers::BOLD),
                ),
                Vec::default(),
            )
            .is_ok());
        app
    }
}

impl Update<Msg> for PicoSmosApp {
    fn update(&mut self, msg: Option<Msg>) -> Option<Msg> {
        if let Some(msg) = msg {
            self.redraw = true;
            match msg {
                Msg::AppClose => {
                    self.quit = true;
                    None
                }
                _ => None,
            }
        } else {
            None
        }
    }
}
