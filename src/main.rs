mod ground;
mod octopus;
mod rabbit;

use crossterm::event::{KeyCode, KeyEventKind};
use ground::Ground;
use octopus::{Cactus, CactusType, ShortCactus, TallCactus};
use rabbit::{RABBIT_ART, RABBIT_HEIGHT, RABBIT_WIDTH, Rabbit};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph},
};
use std::{
    io,
    sync::mpsc::{self, RecvTimeoutError},
    thread,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

enum Event {
    Input(crossterm::event::KeyEvent), // crossterm key input event
    Resize(u16, u16),
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = ratatui::init();

    let (event_tx, event_rx) = mpsc::channel::<Event>();

    let tx_to_input_events = event_tx.clone();
    thread::spawn(move || {
        handle_input_events(tx_to_input_events);
    });

    let mut app = App {
        frame_info: FrameInfo {
            width: terminal.size()?.width,
            height: terminal.size()?.height,
        },
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

struct FrameInfo {
    width: u16,
    height: u16,
}

struct App<'a> {
    frame_info: FrameInfo,
    exit: bool,
    paused: bool,
    ground: Ground,
    rabbit: Rabbit,
    obstacles: Vec<CactusType<'a>>,
}

const TICK_RATE: Duration = Duration::from_millis(16);
const GROUND_SCROLL_SPEED: Duration = Duration::from_millis(16);

impl<'a> App<'a> {
    fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        event_rx: mpsc::Receiver<Event>,
    ) -> io::Result<()> {
        let mut last_tick = Instant::now();

        while !self.exit {
            match event_rx.recv_timeout(TICK_RATE) {
                Ok(Event::Input(key_event)) => self.handle_key_event(key_event)?,
                Ok(Event::Resize(_, _)) => {
                    self.frame_info = FrameInfo {
                        width: 100,
                        height: 21,
                    };
                }
                Err(RecvTimeoutError::Timeout) => {
                    if !self.paused {
                        if last_tick.elapsed() >= GROUND_SCROLL_SPEED {
                            self.on_tick();
                            last_tick = Instant::now();
                        }
                        terminal.draw(|frame| self.draw(frame))?;
                    }
                }
                Err(_) => {}
            }
        }

        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("msg");
        let now_millis = now.as_millis().to_string();

        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(100), Constraint::Min(0)])
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(21), Constraint::Min(0)])
            .split(Rect::new(0, 0, 100, 21));

        frame.render_widget(
            Paragraph::new(format!(
                "Frame: {}, {}",
                self.frame_info.width, self.frame_info.height
            ))
            .block(Block::bordered().title(format!(
                "Rabbit Running, {}, {}, {}",
                now_millis,
                self.ground.offset(),
                self.obstacles.len(),
            ))),
            layout[0],
        );

        let frame_area = frame.area();
        // let ground_base_y = frame_area.height.saturating_sub(3);
        let ground_base_y = 21 - 3;

        let f_width = 100;

        // render ground;
        let visible_slice = self.ground.get_visible_slice(f_width as usize - 2);
        let my_paragraph = Paragraph::new(visible_slice.clone());
        let paragraph_area = Rect::new(1, ground_base_y, f_width, 1);
        frame.render_widget(my_paragraph, paragraph_area);

        // render rabbit;
        let rabbit_paragraph = Paragraph::new(RABBIT_ART);
        let rabbit_top_y = self.rabbit.get_top_y(ground_base_y as f32);
        let rabbit_area = Rect::new(
            self.rabbit.x,
            rabbit_top_y,
            RABBIT_WIDTH,
            RABBIT_HEIGHT as u16, // 兔子的高度
        );
        frame.render_widget(
            rabbit_paragraph.block(Block::default().borders(Borders::NONE)), // 可以移除邊框讓兔子看起來更自然
            rabbit_area,
        );

        // render cactus;

        let filtered_cactus = self
            .obstacles
            .iter()
            .filter(|cactus| match cactus {
                CactusType::Short(short_cactus) => short_cactus.get_all_attr().0 > 0,
                CactusType::Tall(tall_cactus) => tall_cactus.get_all_attr().0 > 0,
            })
            .collect::<Vec<_>>();

        for cactus in filtered_cactus.iter() {
            let (x, width, height, art) = match cactus {
                CactusType::Short(short_cactus) => short_cactus.get_all_attr(), // 如果是 Short，呼叫 short_cactus 的方法
                CactusType::Tall(tall_cactus) => tall_cactus.get_all_attr(), // 如果是 Tall，呼叫 tall_cactus 的方法
            };

            let cactus_paragraph = Paragraph::new(art);
            frame.render_widget(
                cactus_paragraph,
                Rect::new(x, ground_base_y - height, 10, 4),
            );
        }

        let first_cactus = filtered_cactus.first();

        if let Some(cactus) = first_cactus {
            let (x, width, height, _) = match cactus {
                CactusType::Short(short_cactus) => short_cactus.get_all_attr(), // 如果是 Short，呼叫 short_cactus 的方法
                CactusType::Tall(tall_cactus) => tall_cactus.get_all_attr(), // 如果是 Tall，呼叫 tall_cactus 的方法
            };

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

        if self.ground.offset() % 53 == 0 {
            let cactus = CactusType::Short(ShortCactus::new(100 - 3));
            self.obstacles.push(cactus);
        }

        for cactus in self.obstacles.iter_mut() {
            match cactus {
                CactusType::Short(short_cactus) => short_cactus.update_x(1), // 如果是 Short，呼叫 short_cactus 的方法
                CactusType::Tall(tall_cactus) => tall_cactus.update_x(1), // 如果是 Tall，呼叫 tall_cactus 的方法
            };
        }
    }
}

fn handle_input_events(tx: mpsc::Sender<Event>) {
    loop {
        match crossterm::event::read().unwrap() {
            crossterm::event::Event::Key(key_event) => tx.send(Event::Input(key_event)).unwrap(),
            crossterm::event::Event::Resize(width, height) => {
                tx.send(Event::Resize(width, height)).unwrap();
            }
            _ => {}
        }
    }
}
