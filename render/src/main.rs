
use macroquad::{
    experimental::state_machine::State,
    miniquad::{window::screen_size, EventHandler},
    prelude::*,
};
use site::{SiteEvent, SiteState};

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */

mod boids;
mod home;
mod site;

#[macroquad::main("Website")]
async fn main() {
    let mut state = SiteState::new();
    let mut size = (0.0,0.0);

    

    loop {
        state.update();

        if size != screen_size() {
            size = screen_size();
            state.page.event(SiteEvent::Resize {
                rect: Rect::new(0.0, 0.0, size.0, size.1),
            })
        }

        next_frame().await
    }
}
