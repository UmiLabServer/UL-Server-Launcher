use crossterm::event::{Event, KeyEvent};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

pub enum AppEvent {
    Key(KeyEvent),
    Tick,
}

#[derive(Clone)]
pub struct EventConfig {
    pub tick_rate: Duration,
}

impl Default for EventConfig {
    fn default() -> Self {
        Self {
            tick_rate: Duration::from_millis(250),
        }
    }
}

pub struct Events {
    rx: mpsc::Receiver<AppEvent>,
    #[allow(dead_code)]
    input_handle: thread::JoinHandle<()>,
    #[allow(dead_code)]
    tick_handle: thread::JoinHandle<()>,
}

impl Events {
    pub fn new() -> Events {
        Events::with_config(EventConfig::default())
    }

    pub fn with_config(config: EventConfig) -> Events {
        let (tx, rx) = mpsc::channel();
        let tick_rate = config.tick_rate;

        let event_tx = tx.clone();
        let input_handle = thread::spawn(move || {
            loop {
                if crossterm::event::poll(tick_rate).unwrap() {
                    if let Ok(Event::Key(key)) = crossterm::event::read() {
                        if event_tx.send(AppEvent::Key(key)).is_err() {
                            return;
                        }
                    }
                }
            }
        });

        let tick_tx = tx;
        let tick_handle = thread::spawn(move || {
            let mut last_tick = Instant::now();
            loop {
                let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or_else(|| Duration::from_secs(0));

                thread::sleep(timeout);

                if tick_tx.send(AppEvent::Tick).is_err() {
                    break;
                }
                last_tick = Instant::now();
            }
        });

        Events {
            rx,
            input_handle,
            tick_handle,
        }
    }

    pub fn next(&self) -> Result<AppEvent, mpsc::RecvError> {
        self.rx.recv()
    }
}