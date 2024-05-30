use std::default;

use macroquad::{color::{Color, BLACK, PINK}, input::{is_key_down, KeyCode}, math::{Rect, Vec2}, miniquad::window::screen_size, shapes::draw_rectangle, text::draw_text, texture::render_target, window::{clear_background, screen_height, screen_width}};
use macroquad_profiler::ProfilerParams;

use crate::home::Home;


pub enum SiteEvent {
    Resize{rect: Rect}
}


pub trait Page {
    fn render(&mut self);
    fn event(&mut self, event: SiteEvent);
}



pub struct SiteState {
    pub page: Box<dyn Page>,
}


impl SiteState {
    pub fn new() -> Self {
        SiteState {
            page: Box::new(Home::default())
        }
    }
    

    pub fn update(&mut self) {
    
    clear_background(PINK);

    draw_text("text rendering error", screen_width()/4.0, screen_width()/3.0, 20.0, BLACK);

    self.page.render();
    
    if is_key_down(KeyCode::F) {
        draw_rectangle(0.0, screen_height() - 50.0, 100.0, 50.0, BLACK);
        macroquad_profiler::profiler(ProfilerParams {
            fps_counter_pos: Vec2::new(0.0, screen_height() - 100.0),
            ..Default::default()
        })
    }
}
}











