use std::borrow::Borrow;
use sfml::{
    graphics::{Color, CustomShape, CustomShapePoints, RenderTarget, RenderWindow, Shape},
    system::Vector2f,
    window::{Event, Key, Style},
};

use crate::custom_triangle::TriangleShape;

pub struct GameManager<'a> {
    window: RenderWindow,
    shape: CustomShape<'a>,
}

impl<'a> GameManager<'a> {
    pub fn new(x: u32, y: u32, title: String) -> Self {
        GameManager {
            window: RenderWindow::new((x, y), "title", Style::CLOSE, &Default::default()),
            shape: CustomShape::new(Box::new(TriangleShape)),
        }
    }

    pub fn init(&mut self) {
        self.window.set_vertical_sync_enabled(true);

        self.create_shape();
    }

    pub fn run(&mut self) {
        loop {
            while let Some(event) = self.window.poll_event() {
                match event {
                    Event::Closed
                    {

                    } => return,
                    _ => {}
                }
            }

            self.window.clear(Color::BLACK);
            self.window.draw(&self.shape);
            self.window.display();
        }
    }

    pub fn poll_events() {

    }

    fn create_shape(&mut self) {
        //self.shape = CustomShape::new(Box::new(TriangleShape));
        self.shape.set_fill_color(Color::RED);
        self.shape.set_outline_color(Color::GREEN);
        self.shape.set_outline_thickness(3.);
    }
}