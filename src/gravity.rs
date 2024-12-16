use glam::{dvec2, DVec2};

use crate::{
    helper::{plot_arc, HEIGHT, WIDTH},
    ui::dda_line,
    Drawable, AU,
};

const TIME_DELAY: f64 = 0.5;

#[derive(Clone)]
pub struct World {
    pub objects: Vec<PointWeight>,
}

impl World {
    pub fn get_id_offset(&self) -> usize {
        return get_current_id() - self.objects.len();
    }
    pub fn update(&mut self) {
        let secondaries = self.objects.clone();

        // Euler Integration
        // For each object in the world
        for obj in self.objects.iter_mut() {
            let mut net_force = dvec2(0., 0.);

            for secondary in secondaries.clone().into_iter() {
                // Calculate the force of gravity from all other objects
                if secondary.id == obj.id {
                    continue;
                } // If it's not you.

                let grav = obj.gravity(&secondary);

                // Forces are directional
                let directional_vector = (secondary.position - obj.position).normalize() * grav;
                net_force += directional_vector;
            }
            obj.force = net_force;
        }

        for obj in self.objects.iter_mut() {
            // Apply the timestep and Newton's Second
            obj.velocity += (obj.force / obj.mass) * TIME_DELAY;
            obj.position += obj.velocity * TIME_DELAY;
        }
    }
}

impl Drawable for World {
    fn draw(&self, buffer: &mut [u32], zoom: f32, vectors: f32, center: (f64, f64)) {
        self.objects.clone().into_iter().for_each(|obj| {
            obj.draw(buffer, zoom, 0., center);
        });

        if vectors % 3. == 0. {
            self.objects.clone().into_iter().for_each(|obj| {
                let au_factor: f64 = 1.496e8 / zoom as f64;
                const CENTER: DVec2 = dvec2((WIDTH as f64) / 2., HEIGHT as f64 / 2.);

                let x = (obj.position.x.floor() / au_factor) + CENTER.x + center.0;
                let y = (obj.position.y.floor() / au_factor) + CENTER.y + center.1;

                let factor = (obj.velocity / obj.force).normalize().abs().element_sum();

                dda_line(
                    buffer,
                    x,
                    y,
                    x + (obj.force.normalize() * obj.radius).x,
                    y + (obj.force.normalize() * obj.radius).y,
                    0xffff0000,
                ); // force

                dda_line(
                    buffer,
                    x,
                    y,
                    x + (obj.velocity.normalize() * obj.radius * factor).x,
                    y + (obj.velocity.normalize() * obj.radius * factor).y,
                    0xff00ff00,
                );
            });
        }

        if vectors % 5. == 0. {
            let mut point = PointWeight::new(dvec2(0., 0.), 50., 0x00000000, 0.);

            let x_divs = 30;
            let y_divs = 20;
            for x_factor in (-(x_divs / 2))..(x_divs / 2) {
                for y_factor in (-(y_divs / 2))..(y_divs / 2) {
                    let au_factor: f64 = 1.496e8 / zoom as f64;

                    let x_factor = x_factor as f64;
                    let y_factor = y_factor as f64;
                    let x_divs = x_divs as f64;
                    let y_divs = y_divs as f64;

                    point.position = dvec2(
                        ((x_factor / x_divs) * (WIDTH as f64) * au_factor) + center.0,
                        ((y_factor / y_divs) * (HEIGHT as f64) * au_factor) + center.1,
                    );

                    let mut net_force = dvec2(0., 0.);

                    for secondary in self.objects.clone().into_iter() {
                        let grav = point.gravity(&secondary);

                        // Forces are directional
                        let directional_vector =
                            (secondary.position - point.position).normalize() * grav;
                        net_force += directional_vector;
                    }
                    point.force = net_force;

                    const CENTER: DVec2 = dvec2((WIDTH as f64) / 2., HEIGHT as f64 / 2.);

                    let x = (point.position.x.floor() / au_factor) + CENTER.x + center.0;
                    let y = (point.position.y.floor() / au_factor) + CENTER.y + center.1;

                    // println!("{}m{}", x, y);

                    dda_line(
                        buffer,
                        x,
                        y,
                        x + (point.force.normalize() * 20.).x,
                        y + (point.force.normalize() * 20.).y,
                        0x60606060,
                    );
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct PointWeight {
    pub id: usize,
    pub velocity: DVec2,
    pub force: DVec2,
    pub position: DVec2,
    pub mass: f64,
    pub radius: f64,
    pub color: u32,
}

use std::sync::atomic::AtomicUsize;

static COUNTER: AtomicUsize = AtomicUsize::new(0);

fn id() -> usize {
    COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}

fn get_current_id() -> usize {
    COUNTER.load(std::sync::atomic::Ordering::Relaxed)
}

pub fn generate_semi_random_u32(mut id: usize) -> u32 {
    // Constants for the linear congruential generator
    let mut id = id as u32;
    const A: u32 = 1664525;
    const C: u32 = 1013904223;

    // Apply LCG formula: Xₙ₊₁ = (A * Xₙ + C) % 2³²
    id = id.wrapping_mul(A).wrapping_add(C);

    // Introduce further variability by running the LCG a few extra iterations
    id = id.wrapping_mul(A).wrapping_add(C);
    id = id.wrapping_mul(A).wrapping_add(C);

    id
}

impl PointWeight {
    pub fn new(position: DVec2, mass: f64, mut color: u32, radius: f64) -> PointWeight {
        let id = id();

        if color == 0xfafafafa {
            color = generate_semi_random_u32(id);
        }

        PointWeight {
            id: id,
            velocity: dvec2(0., 0.),
            force: dvec2(0., 0.),
            position,
            mass,
            radius,
            color,
        }
    }

    pub fn gravity(&self, other: &PointWeight) -> f64 {
        const G: f64 = 6.67e-11; // Gravitational Constant
                                 // Distance is at least 10e8 m, because otherwise...
        let dist = self.position.distance(other.position).max(10000000.);

        (G * self.mass * other.mass) / dist.powi(2) // The equation for gravity!
    }
}

impl Drawable for PointWeight {
    fn draw(&self, frame: &mut [u32], zoom: f32, _: f32, center: (f64, f64)) {
        let au_factor: f64 = 1.496e8 / zoom as f64;
        const CENTER: DVec2 = dvec2((WIDTH as f64) / 2., HEIGHT as f64 / 2.);

        let x = (self.position.x.floor() / au_factor) + CENTER.x + center.0;
        let y = (self.position.y.floor() / au_factor) + CENTER.y + center.1;

        plot_arc(frame, x, y, self.radius, self.color, true, true, true, true);
    }
}
