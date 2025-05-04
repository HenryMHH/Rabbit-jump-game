use crate::constants::game::{GROUND_SHIFT, GameState};
use crate::constants::timing::{GROUND_SCROLL_SPEED, TICK_RATE};
use crate::core::{
    cactus::{CactusType, Obstacle},
    ground::Ground,
    rabbit::{RABBIT_ART, RABBIT_HEIGHT, RABBIT_WIDTH, Rabbit},
};
use crate::utils::{CustomEvent, get_new_obstacle};
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph},
};
use std::sync::mpsc::{self, RecvTimeoutError};
use std::time::Instant;

pub struct App<'a> {
    pub frame_rect: Rect,
    pub state: GameState,
    pub ground: Ground,
    pub rabbit: Rabbit,
    pub obstacles: Vec<CactusType<'a>>,
}

impl<'a> App<'a> {
    pub fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        event_rx: mpsc::Receiver<CustomEvent>,
    ) -> Result<()> {
        let mut last_tick = Instant::now();

        while self.state != GameState::Quit {
            match event_rx.recv_timeout(TICK_RATE) {
                Ok(CustomEvent::Input(key_event)) => self.handle_key_event(key_event)?,
                Ok(CustomEvent::Resize(width, height)) => {
                    self.frame_rect = Rect::new(0, 0, width, height);
                }
                Err(RecvTimeoutError::Timeout) => {
                    if self.state == GameState::GameOver {
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

        // render cactus
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
    }

    fn check_collisions(&mut self) {
        let ground_base_y = self.frame_rect.height.saturating_sub(GROUND_SHIFT);
        let rabbit_top_y = self.rabbit.get_top_y(ground_base_y as f32);
        let rabbit_area = Rect::new(
            self.rabbit.x,
            rabbit_top_y,
            RABBIT_WIDTH,
            RABBIT_HEIGHT as u16,
        );
        let first_cactus = self.obstacles.first();

        if let Some(cactus) = first_cactus {
            let (x, width, height, _) = match cactus {
                CactusType::Short(short_cactus) => short_cactus.get_all_attr(),
                CactusType::Tall(tall_cactus) => tall_cactus.get_all_attr(),
            };

            // x + 1, ground_base_y - height - 1, slightly shift is for optimizing the collision detection
            let cactus_area = Rect::new(x + 1, ground_base_y - height - 1, width, height);

            if rabbit_area.intersects(cactus_area) {
                self.state = GameState::GameOver;
            }
        }
    }

    fn remove_obstacle(&mut self) {
        if !self.obstacles.is_empty() {
            // remove cactus if the x is less or equal to 0
            self.obstacles.retain(|cactus| match cactus {
                CactusType::Short(short_cactus) => short_cactus.get_all_attr().0 > 0,
                CactusType::Tall(tall_cactus) => tall_cactus.get_all_attr().0 > 0,
            });
        }
    }

    fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> Result<()> {
        if key_event.kind == KeyEventKind::Press {
            match key_event.code {
                KeyCode::Char('q') => {
                    self.state = GameState::Quit;
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
        self.remove_obstacle();
        self.ground.scroll(None);
        self.rabbit.update_physics();
        self.check_collisions();

        // add cactus every 53 ticks
        if self.ground.offset() % 53 == 0 {
            let new_obstacle = get_new_obstacle(self.frame_rect.width);

            self.obstacles.push(new_obstacle);
        }

        for cactus in self.obstacles.iter_mut() {
            match cactus {
                CactusType::Short(short_cactus) => short_cactus.update_x(1), // 如果是 Short，呼叫 short_cactus 的方法
                CactusType::Tall(tall_cactus) => tall_cactus.update_x(1), // 如果是 Tall，呼叫 tall_cactus 的方法
            };
        }
    }
}
