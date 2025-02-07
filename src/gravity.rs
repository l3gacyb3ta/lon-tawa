use glam::{dvec2, DVec2};

use crate::{
    helper::{plot_arc, plot_pixel, HEIGHT, WIDTH},
    ui::dda_line,
    Drawable,
};

const TIME_DELAY: f64 = 1.;
const WIDTH_F64: f64 = WIDTH as f64;
const HEIGHT_F64: f64 = HEIGHT as f64;

#[derive(Clone)]
pub struct World {
    pub objects: Vec<PointWeight>,
    // a type of object that doesn't affect others but does fly through space
    pub particles: Vec<Particle>,
}

impl World {
    pub fn get_id_offset(&self) -> usize {
        return get_current_id() - self.objects.len();
    }

    /// Simulate the world ahead a number of steps, returning the positions of the object.
    pub fn simulate_ahead(
        &self,
        obj: PointWeight,
        steps: usize,
        clairvoyant: bool,
        speed_factor: f64,
    ) -> Vec<DVec2> {
        let mut new_obj = obj.clone();
        let mut objects = self.objects.clone();
        let time_factor = TIME_DELAY * speed_factor;
        objects.push(new_obj);

        let mut positions = Vec::new();

        for _ in 0..steps {
            let secondaries = objects.clone();

            // Leapfrog integration
            // Step 1: First half-step position update using current velocities
            for obj in objects.iter_mut() {
                if !clairvoyant && obj.id != new_obj.id {
                    continue;
                }

                if obj.velocity == dvec2(0.420, -0.420) {
                    continue;
                }

                obj.position += obj.velocity * (time_factor / 2.0);
            }

            // For each object in the world
            for obj in objects.iter_mut() {
                if obj.velocity == dvec2(0.420, -0.420) {
                    continue;
                }
                if !clairvoyant && obj.id != new_obj.id {
                    continue;
                }

                let mut net_force = dvec2(0., 0.);

                for secondary in &secondaries {
                    if secondary.id == obj.id {
                        continue;
                    }

                    let grav = obj.gravity(&secondary);
                    let directional_vector = (secondary.position - obj.position).normalize() * grav;
                    net_force += directional_vector;
                }

                obj.force = net_force;
                obj.velocity += (obj.force / obj.mass) * time_factor;
            }

            // Step 3: Second half-step position update using new velocities
            for obj in objects.iter_mut() {
                if obj.velocity == dvec2(0.420, -0.420) {
                    continue;
                }
                if !clairvoyant && obj.id != new_obj.id {
                    continue;
                }

                obj.position += obj.velocity * (time_factor / 2.0);

                if obj.id == new_obj.id {
                    positions.push(obj.position);
                }
            }
        }

        positions
    }

    pub fn simulate_ahead_all(&self, steps: usize, speed_factor: f64) -> Vec<Vec<DVec2>> {
        let mut objects = self.objects.clone();
        let time_factor = TIME_DELAY * speed_factor * 2.;

        let mut positions = Vec::new();
        for obj in objects.clone() {
            positions.push(vec![obj.position]);
        }

        for _ in 0..steps {
            let secondaries = objects.clone();
            // Leapfrog integration
            // Step 1: First half-step position update using current velocities
            for obj in objects.iter_mut() {
                if obj.velocity == dvec2(0.420, -0.420) {
                    continue;
                }

                obj.position += obj.velocity * (time_factor / 2.0);
            }

            // For each object in the world
            for obj in objects.iter_mut() {
                if obj.velocity == dvec2(0.420, -0.420) {
                    continue;
                }

                let mut net_force = dvec2(0., 0.);

                for secondary in &secondaries {
                    if secondary.id == obj.id {
                        continue;
                    }

                    let grav = obj.gravity(&secondary);
                    let directional_vector = (secondary.position - obj.position).normalize() * grav;
                    net_force += directional_vector;
                }

                obj.force = net_force;
                obj.velocity += (obj.force / obj.mass) * time_factor;
            }

            // Step 3: Second half-step position update using new velocities
            let mut i = -1;
            for obj in objects.iter_mut() {
                i += 1;
                if obj.velocity == dvec2(0.420, -0.420) {
                    continue;
                }

                obj.position += obj.velocity * (time_factor / 2.0);

                positions[i as usize].push(obj.position);
            }
        }

        positions
    }

    pub fn update_particles(&mut self, speed_factor: f64) {
        let time_factor = TIME_DELAY * speed_factor;

        // Step 1: First half-step position update using current velocities
        for particle in self.particles.iter_mut() {
            particle.position += particle.velocity * (time_factor / 2.0);
        }

        let mut net_force: DVec2;
        for particle in self.particles.iter_mut() {
            net_force = dvec2(0., 0.);
            for object in self.objects.iter_mut() {
                let grav = particle.gravity(*object);
                let directional_vector = (object.position - particle.position).normalize() * grav;
                net_force += directional_vector;
            }

            particle.force = net_force;
            particle.velocity += (particle.force / particle.mass) * time_factor;
        }

        for particle in self.particles.iter_mut() {
            particle.position += particle.velocity * (time_factor / 2.0);
        }
    }

    pub fn update(&mut self, speed_factor: f64) {
        let secondaries = self.objects.clone();

        let time_factor = TIME_DELAY * speed_factor;

        // Leapfrog integration
        // Step 1: First half-step position update using current velocities
        for obj in self.objects.iter_mut() {
            if obj.velocity == dvec2(0.420, -0.420) {
                continue;
            }

            obj.position += obj.velocity * (time_factor / 2.0);
        }

        // For each object in the world
        for obj in self.objects.iter_mut() {
            if obj.velocity == dvec2(0.420, -0.420) {
                continue;
            }

            let mut net_force = dvec2(0., 0.);

            for secondary in &secondaries {
                if secondary.id == obj.id {
                    continue;
                }

                let grav = obj.gravity(&secondary);
                let directional_vector = (secondary.position - obj.position).normalize() * grav;
                net_force += directional_vector;
            }

            obj.force = net_force;
            obj.velocity += (obj.force / obj.mass) * time_factor;
        }

        // Step 3: Second half-step position update using new velocities
        for obj in self.objects.iter_mut() {
            if obj.velocity == dvec2(0.420, -0.420) {
                continue;
            }

            obj.position += obj.velocity * (time_factor / 2.0);
        }

        self.update_particles(speed_factor);
    }
}

impl Drawable for World {
    fn draw(&self, buffer: &mut [u32], zoom: f32, vectors: f32, center: (f64, f64)) {
        let mut point = PointWeight {
            id: 0,
            velocity: dvec2(0., 0.),
            force: dvec2(0., 0.),
            position: dvec2(0., 0.),
            mass: 50.,
            radius: 0.,
            color: 0x00000000,
        };

        let au_factor: f64 = 1.496e8 / zoom as f64;
        const CENTER: DVec2 = dvec2((WIDTH_F64) / 2., HEIGHT_F64 / 2.);
        let grid_spacing = au_factor * 30.; // 50 pixels per grid line

        // Calculate how many grid cells we need based on viewport size
        const VIEWPORT_WIDTH: f64 = WIDTH_F64;
        const VIEWPORT_HEIGHT: f64 = HEIGHT_F64;

        let half_viewport_width_worldspace = (VIEWPORT_WIDTH / 2.0) * au_factor;
        let half_viewport_height_worldspace = (VIEWPORT_HEIGHT / 2.0) * au_factor;

        let center_worldspace = dvec2(center.0, center.1) * au_factor;

        let spacings_from_origin_on_left =
            ((center_worldspace.x + half_viewport_width_worldspace) / grid_spacing).floor();
        let spacings_from_origin_on_right =
            ((center_worldspace.x - half_viewport_width_worldspace) / grid_spacing).ceil();

        let spacings_from_origin_on_top =
            ((center_worldspace.y + half_viewport_height_worldspace) / grid_spacing).floor();
        let spacings_from_origin_on_bottom =
            ((center_worldspace.y - half_viewport_height_worldspace) / grid_spacing).ceil();

        for x_spacings_worldspace in
            (spacings_from_origin_on_right as i32)..=(spacings_from_origin_on_left as i32)
        {
            for y_spacings_worldspace in
                (spacings_from_origin_on_bottom as i32)..=(spacings_from_origin_on_top as i32)
            {
                let point_worldspace = dvec2(
                    x_spacings_worldspace as f64 * grid_spacing,
                    y_spacings_worldspace as f64 * grid_spacing,
                );

                let new_point = (point_worldspace - center_worldspace) / au_factor;
                // the units of new_point are pixels from the center of the screen

                let new_point = new_point * dvec2(-1., -1.);

                let x = CENTER.x + new_point.x;
                let y = CENTER.y + new_point.y;

                // plot the forcefield
                if vectors % 5. == 0. {
                    point.position = point_worldspace * dvec2(-1., -1.);

                    let mut net_force = dvec2(0., 0.);
                    for secondary in self.objects.clone().into_iter() {
                        let grav = point.gravity(&secondary);

                        // Forces are directional
                        let directional_vector =
                            (secondary.position - point.position).normalize() * grav;
                        net_force += directional_vector;
                    }

                    dda_line(
                        buffer,
                        x,
                        y,
                        x + (net_force.normalize() * 5.).x,
                        y + (net_force.normalize() * 5.).y,
                        0x60606060,
                    );
                } else {
                    plot_pixel(buffer, x as usize, y as usize, 0xc0c0c0c0);
                }
            }
        }

        self.objects.clone().into_iter().for_each(|obj| {
            obj.draw(buffer, zoom, 0., center);
        });

        self.particles.iter().for_each(|particle| {
            let x = (particle.position.x.floor() / au_factor) + CENTER.x + center.0;
            let y = (particle.position.y.floor() / au_factor) + CENTER.y + center.1;

            if x > WIDTH_F64 || x < 0. || y > HEIGHT_F64 || y < 0. {
                return;
            }

            plot_arc(buffer, x, y, 1., 0xa0a0a0a0, true, false, true, false);
        });

        if vectors % 3. == 0. {
            self.objects.clone().into_iter().for_each(|obj| {
                let au_factor: f64 = 1.496e8 / zoom as f64;
                const CENTER: DVec2 = dvec2((WIDTH_F64) / 2., HEIGHT_F64 / 2.);

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

#[derive(Clone, Copy, Debug)]
pub struct Particle {
    pub velocity: DVec2,
    pub force: DVec2,
    pub position: DVec2,
    pub mass: f64,
    pub color: u32,
}

impl Particle {
    pub fn new(position: DVec2, mass: f64, color: u32) -> Self {
        Self {
            velocity: DVec2 { x: 0., y: 0. },
            force: DVec2 { x: 0., y: 0. },
            position,
            mass,
            color,
        }
    }

    pub fn new_with_vel(position: DVec2, mass: f64, color: u32, velocity: DVec2) -> Self {
        Self {
            velocity,
            force: DVec2 { x: 0., y: 0. },
            position,
            mass,
            color,
        }
    }

    pub fn gravity(&self, other: PointWeight) -> f64 {
        const G: f64 = 6.67e-11; // Gravitational Constant
                                 // Distance is at least 10e8 m, because otherwise...
        let dist = self.position.distance(other.position);

        (G * self.mass * other.mass) / dist.powi(2) // The equation for gravity!
    }
}

use std::sync::atomic::AtomicUsize;

static COUNTER: AtomicUsize = AtomicUsize::new(0);

fn id() -> usize {
    COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}

fn get_current_id() -> usize {
    COUNTER.load(std::sync::atomic::Ordering::Relaxed)
}

pub fn generate_semi_random_u32(id: usize) -> u32 {
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

    pub fn new_with_vel(
        position: DVec2,
        vel: DVec2,
        mass: f64,
        mut color: u32,
        radius: f64,
        id_override: usize,
    ) -> PointWeight {
        let id = if id_override == 12062006 {
            id_override
        } else {
            id()
        };

        if color == 0xfafafafa {
            color = generate_semi_random_u32(id);
        }

        PointWeight {
            id: id,
            velocity: vel,
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
        let dist = self.position.distance(other.position).max(1000.);

        (G * self.mass * other.mass) / dist.powi(2) // The equation for gravity!
    }
}

impl Drawable for PointWeight {
    fn draw(&self, frame: &mut [u32], zoom: f32, _: f32, center: (f64, f64)) {
        let au_factor: f64 = 1.496e8 / zoom as f64;
        const CENTER: DVec2 = dvec2((WIDTH_F64) / 2., HEIGHT_F64 / 2.);

        let x = (self.position.x.floor() / au_factor) + CENTER.x + center.0;
        let y = (self.position.y.floor() / au_factor) + CENTER.y + center.1;

        plot_arc(frame, x, y, self.radius, self.color, true, true, true, true);
    }
}
