use macroquad::{ prelude::*};


const INFLUENCE_RADIUS: f32 = 20.0;
const PERSONAL_BUBBLE: f32 = 15.0;
const VELOCITY: f32 = 180.0;
const RAND: f32 = 20.0;
const WALL: f32 = 30.0;
const SPEED: f32 = 0.15;

const SEPARATION: f32 = 1.0;
const FOLLOW: f32 = 6.0;
const ATTRACTION: f32 = 3.0;

// inverse relationship
const SPEED_VARIATION: f32 = 1.8;

pub struct BoidSim {
    boids: Vec<Boid>,
    boid_area: f32,
}

impl BoidSim {
    pub fn new(size: f32) -> BoidSim {
        let boids: Vec<Boid> = (0..600).map(|_| Boid::new_random(size)).collect();


        return BoidSim {
            boids,
            boid_area: size,
        }
    }

    fn cam_pos(&self) -> Vec3A {

        return vec3(80.0 , self.boid_area , 20.).into();
    }
}

impl BoidSim {
    pub fn render_background(&self) {

        // println!("{}",self.boids.len());

        set_camera(&Camera3D {
            position: self.cam_pos().into(),
            up: vec3(0. , 0. , 1.),
            target: vec3(0., 0., 0.),
            projection: Projection::Perspective,
            ..Default::default()
        });



        // draw_cube_wires(
        //     Vec3::new(0.0 , 0.0, 0.0),
        //     Vec3::splat(self.boid_area),
        //     RED,
        // );

        for boid in &self.boids {
            if  boid.pos.distance(self.cam_pos()) > self.boid_area {
                boid.render(self.boid_area);
            }
        }

        set_default_camera();
    }

    pub fn update_boids(&mut self) {
        
        let mut pre = (0..self.boids.len()).map(|_| Boid::blank()).collect::<Vec<Boid>>();
        pre.copy_from_slice(&self.boids.as_slice());

        for boid in self.boids.iter_mut() {
            boid.update(&pre, self.boid_area);
        }

    }

    pub fn render_foreground(&self) {
        set_camera(&Camera3D {
            position: vec3(80.0 , self.boid_area, 20.),
            up: vec3(0. , 0. , 1.),
            target: vec3(0., 0., 0.),
            projection: Projection::Perspective,
            ..Default::default()
        });

        let cube_max = Vec3::splat(self.boid_area);

        
        for boid in &self.boids {
            if  boid.pos.distance(self.cam_pos()) < self.boid_area * 2.0 {
                boid.render(self.boid_area);
            }
        }
        // draw_line_3d(cube_max*0.5, Vec3::new(cube_max.x/2.0, cube_max.y/2.0, -cube_max.z * 0.5), RED);
        
        set_default_camera();
    }
}

#[derive(Clone, Copy)]
pub struct Boid {
    pos: Vec3A,
    velocity: Vec3A,
    speed: f32,
}

impl Boid {

    fn blank() -> Boid {
        Boid {
            pos: Vec3A::ZERO,
            velocity: Vec3A::ONE,
            speed: 0.0,
        }
    }

    fn render(&self, size: f32) {
        draw_line_3d(self.pos.into(), (self.pos + self.velocity*10.0 * size/500.0).into(), Color::from_rgba(20, 20, 80, 100));
    }

    fn new_random(area_size: f32) -> Boid {
        Boid {
            pos: Vec3A::new(
                (fastrand::f32() - 0.5) * area_size * 10.0,
                (fastrand::f32() - 0.5) * area_size * 10.0,
                (fastrand::f32() - 0.5) * area_size * 10.0,
            ),
            velocity: Vec3A::new(
                fastrand::f32() - 0.5,
                fastrand::f32() - 0.5,
                fastrand::f32() - 0.5,
            )
            .normalize(),
            speed: 1.0 + fastrand::f32() * 1.0/SPEED_VARIATION

        }
    }


    fn update(&mut self, nearby: &[Boid], size: f32) {
        
        let variation = Vec3A::new(
            (fastrand::f32() - 0.5),
            (fastrand::f32() - 0.5),
            (fastrand::f32() - 0.5),
        )
        .normalize();

        let wall = Vec3A::splat(size/2.0 - 1.0) - self.pos.abs();
        
        let return_center = vec3a(
            wall.x.min(0.0) * self.pos.x.signum(),
            wall.y.min(0.0) * self.pos.y.signum(),
            wall.z.min(0.0) * self.pos.z.signum(),
        );
        
        let mut separation: Vec<Vec3A> = vec![];
        let mut follow:Vec<Vec3A> = vec![];
        let mut attraction:Vec<Vec3A> = vec![];

        for boid in nearby {
            let distance = boid.pos.distance(self.pos);

            if distance < 0.0001 {
                continue;
            }

            let direction = boid.pos - self.pos;

            if distance < PERSONAL_BUBBLE * size/500.0 {
                separation.push(direction - direction.normalize() * PERSONAL_BUBBLE * size/500.0);
            }
            if distance < INFLUENCE_RADIUS * size/500.0 && distance > PERSONAL_BUBBLE * size/500.0 {
                attraction.push(direction.normalize() * (INFLUENCE_RADIUS * size/500.0) - direction);
                follow.push(boid.velocity * (direction.normalize() * (INFLUENCE_RADIUS * size/500.0) - direction).length().powf(1.5))
            }

        }
        let separation_sum = if separation.len() > 0 {separation.iter().sum::<Vec3A>() / separation.len() as f32} else {Vec3A::ZERO};
        let attraction_sum = if attraction.len() > 0 {attraction.iter().sum::<Vec3A>() / (attraction.len() as f32 * 1.6)} else {Vec3A::ZERO};
        let follow_sum = if follow.len() > 0 {follow.iter().sum::<Vec3A>() / (follow.len() as f32 * 2.0)} else {Vec3A::ZERO};

        self.velocity = 
            VELOCITY * self.velocity +
            RAND * variation + 
            WALL * return_center + 
            SEPARATION * separation_sum +
            ATTRACTION * attraction_sum + 
            FOLLOW * follow_sum 
        ;

        self.velocity = self.velocity.normalize();

        self.pos += self.velocity * SPEED * get_frame_time() * size * self.speed;

    }

}
