use crate::boids::BoidSim;
use crate::site::{Page, SiteEvent};
use macroquad::prelude::*;
use macroquad::ui::root_ui;


struct Textures {
    github: Texture2D,
    itch: Texture2D,
}

impl Default for Textures {
    fn default() -> Self {
        Self { 
            github: Texture2D::from_file_with_format(include_bytes!("img/github.png"), None),
            itch: Texture2D::from_file_with_format(include_bytes!("img/itch.png"), None),
            // other: Texture2D::from_file_with_format(include_bytes!("img/other.png"), None),
        }
    }
}

pub struct Home {
    title_font: Option<Font>,
    // cube_texture: Texture2D,
    boid_sim: BoidSim,
    textures: Textures,
}

impl Default for Home {
    fn default() -> Self {
        Home {
            title_font: load_ttf_font_from_bytes(include_bytes!("fonts/BluuNext-Bold.ttf")).ok(),
            // cube_texture: Texture2D::from_file_with_format(include_bytes!("img/sun.png"), None),
            boid_sim: BoidSim::new(100.0),
            textures: Textures::default()
        }
    }
}

impl Page for Home {
    fn render(&mut self) {

        let mouse = mouse_position();

        self.boid_sim.update_boids();
        
        set_default_camera();
        let scale = screen_width() / 200.0;

        clear_background(WHITE);

        self.boid_sim.render_background();
        let font_size = (25.0 * scale) as u16;
        let center = get_text_center("Ollie Lynas", self.title_font.as_ref(), font_size, 1.0, 0.0);
        draw_text_ex(
            "Ollie Lynas",
            screen_width()/2.0 - center.x,
            screen_height() / 2.5,
            TextParams {
                font: self.title_font.as_ref(),
                color: BLACK,
                font_size: font_size,
                ..Default::default()
            },
        );

        let image_scale = scale/10.0;

        // let itch_rect = Rect::new(screen_width() * 1.0/3.0 - self.textures.itch.size().x * image_scale/2.0, 
        // screen_height() * 2.0/3.0, self.textures.itch.size().x * image_scale, self.textures.itch.size().y * image_scale);
        // let github_rect = Rect::new(screen_width() * 1.0/2.0 - self.textures.github.size().x * image_scale/5.0, 
        // screen_height() * 2.0/3.0, self.textures.github.size().x * image_scale, self.textures.github.size().y * image_scale);

        // draw_texture_ex(&self.textures.itch, itch_rect.x, itch_rect.y, 
        // WHITE, DrawTextureParams {
        //     dest_size: Some(itch_rect.size()),
        //     ..Default::default()
        // });
        // draw_texture_ex(&self.textures.github, github_rect.x, github_rect.y, 
        // WHITE, DrawTextureParams {
        //     dest_size: Some(github_rect.size()),
        //     ..Default::default()
        // });
        
        
        
        

        self.boid_sim.render_foreground();



    }


    fn event(&mut self, event: SiteEvent) {


        match event {
            SiteEvent::Resize { rect } => {
                self.boid_sim = BoidSim::new(rect.w / 0.8);
            },
        }

    }
}
