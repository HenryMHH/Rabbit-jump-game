mod constants;
mod ground;
mod octopus;
mod rabbit;
mod utils;

use constants::Size::MAX_FRAME_SIZE;
use crossterm::event::{KeyCode, KeyEventKind};
use ground::Ground;
use octopus::{Cactus, CactusType, ShortCactus, TallCactus};
use rabbit::{RABBIT_ART, RABBIT_HEIGHT, RABBIT_WIDTH, Rabbit};
use rand::random;
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph},
};
use std::{
    io,
    sync::mpsc::{self, RecvTimeoutError},
    thread,
    time::{Duration, Instant},
};
use utils::get_default_frame_rect;

enum CustomEvent {
    Input(crossterm::event::KeyEvent),
    Resize(u16, u16),
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = ratatui::init();

    let (event_tx, event_rx) = mpsc::channel::<CustomEvent>();

    let tx_to_input_events = event_tx.clone();
    thread::spawn(move || {
        handle_input_events(tx_to_input_events);
    });

    let terminal_rect = get_default_frame_rect(&terminal);

    let mut app = App {
        frame_rect: terminal_rect,
        exit: false,
        paused: false,
        ground: Ground::new("-^-.-^.".to_string()),
        rabbit: Rabbit::new(),
        obstacles: vec![],
    };
    app.run(&mut terminal, event_rx)?;

    ratatui::restore();
    Ok(())
}

const TICK_RATE: Duration = Duration::from_millis(16);
const GROUND_SCROLL_SPEED: Duration = Duration::from_millis(16);
const GROUND_SHIFT: u16 = 3;

struct App<'a> {
    frame_rect: Rect,
    exit: bool,
    paused: bool,
    ground: Ground,
    rabbit: Rabbit,
    obstacles: Vec<CactusType<'a>>,
}

impl<'a> App<'a> {
    fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        event_rx: mpsc::Receiver<CustomEvent>,
    ) -> io::Result<()> {
        let mut last_tick = Instant::now();

        while !self.exit {
            match event_rx.recv_timeout(TICK_RATE) {
                Ok(CustomEvent::Input(key_event)) => self.handle_key_event(key_event)?,
                Ok(CustomEvent::Resize(width, height)) => {
                    self.frame_rect = Rect::new(0, 0, width, height);
                }
                Err(RecvTimeoutError::Timeout) => {
                    if self.paused {
                        continue;
                    };

                    if last_tick.elapsed() >= GROUND_SCROLL_SPEED {
                        self.on_tick();
                        last_tick = Instant::now();
                    }
                    terminal.draw(|frame| self.draw(frame))?;
                }
                Err(_) => {}
            }
        }

        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        // define layout
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(self.frame_rect.width),
                Constraint::Min(0),
            ])
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(self.frame_rect.height),
                Constraint::Min(0),
            ])
            .split(Rect::new(
                0,
                0,
                self.frame_rect.width,
                self.frame_rect.height,
            ));

        let main_block = layout[0];

        frame.render_widget(
            Block::bordered().title(format!("Rabbit Jumping")),
            main_block,
        );

        let ground_base_y = self.frame_rect.height.saturating_sub(GROUND_SHIFT);

        // render ground;
        let x_border_width = 2;
        let ground_slice_width = self.frame_rect.width - x_border_width;
        let visible_slice = self.ground.get_visible_slice(ground_slice_width as usize);
        let my_paragraph = Paragraph::new(visible_slice.clone());
        let paragraph_area = Rect::new(1, ground_base_y, ground_slice_width, 1);
        frame.render_widget(my_paragraph, paragraph_area);

        // render rabbit;
        let rabbit_paragraph = Paragraph::new(RABBIT_ART);
        let rabbit_top_y = self.rabbit.get_top_y(ground_base_y as f32);
        let rabbit_area = Rect::new(
            self.rabbit.x,
            rabbit_top_y,
            RABBIT_WIDTH,
            RABBIT_HEIGHT as u16,
        );
        frame.render_widget(
            rabbit_paragraph.block(Block::default().borders(Borders::NONE)), // 可以移除邊框讓兔子看起來更自然
            rabbit_area,
        );

        // remove cactus if the x is less or equal to 0
        self.obstacles.retain(|cactus| match cactus {
            CactusType::Short(short_cactus) => short_cactus.get_all_attr().0 > 0,
            CactusType::Tall(tall_cactus) => tall_cactus.get_all_attr().0 > 0,
        });

        for cactus in self.obstacles.iter() {
            let (x, width, height, art) = match cactus {
                CactusType::Short(short_cactus) => short_cactus.get_all_attr(),
                CactusType::Tall(tall_cactus) => tall_cactus.get_all_attr(),
            };

            let cactus_paragraph = Paragraph::new(art);
            frame.render_widget(
                cactus_paragraph,
                Rect::new(x, ground_base_y - height, width, height),
            );
        }

        let first_cactus = self.obstacles.first();

        if let Some(cactus) = first_cactus {
            let (x, width, height, _) = match cactus {
                CactusType::Short(short_cactus) => short_cactus.get_all_attr(),
                CactusType::Tall(tall_cactus) => tall_cactus.get_all_attr(),
            };

            // x + 1, ground_base_y - height - 1, slightly shift is for optimizing the collision detection
            let cactus_area = Rect::new(x + 1, ground_base_y - height - 1, width, height);

            if rabbit_area.intersects(cactus_area) {
                self.paused = true;
            }
        }
    }

    fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> io::Result<()> {
        if key_event.kind == KeyEventKind::Press {
            match key_event.code {
                KeyCode::Char('q') => {
                    self.exit = true;
                }
                KeyCode::Char(' ') => {
                    self.rabbit.jump();
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn on_tick(&mut self) {
        self.ground.scroll();
        self.rabbit.update_physics();

        // add cactus every 53 ticks
        if self.ground.offset() % 53 == 0 {
            add_cactus(&mut self.obstacles, self.frame_rect.width);
        }

        for cactus in self.obstacles.iter_mut() {
            match cactus {
                CactusType::Short(short_cactus) => short_cactus.update_x(1), // 如果是 Short，呼叫 short_cactus 的方法
                CactusType::Tall(tall_cactus) => tall_cactus.update_x(1), // 如果是 Tall，呼叫 tall_cactus 的方法
            };
        }
    }
}

fn add_cactus(obstacles: &mut Vec<CactusType>, frame_width: u16) {
    let cactus = match random::<u8>() % 2 {
        0 => CactusType::Short(ShortCactus::new(frame_width)),
        // _ => CactusType::Short(ShortCactus::new(frame_width)),
        _ => CactusType::Tall(TallCactus::new(frame_width)),
    };
    obstacles.push(cactus);
}

fn handle_input_events(tx: mpsc::Sender<CustomEvent>) {
    loop {
        match crossterm::event::read().unwrap() {
            crossterm::event::Event::Key(key_event) => {
                tx.send(CustomEvent::Input(key_event)).unwrap()
            }
            crossterm::event::Event::Resize(width, height) => {
                let f_w = if width > MAX_FRAME_SIZE.0 {
                    MAX_FRAME_SIZE.0
                } else {
                    width
                };
                let f_h = if height > MAX_FRAME_SIZE.1 {
                    MAX_FRAME_SIZE.1
                } else {
                    height
                };

                tx.send(CustomEvent::Resize(f_w, f_h)).unwrap();
            }
            _ => {}
        }
    }
}
