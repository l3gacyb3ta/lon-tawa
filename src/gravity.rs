use glam::{dvec2, DVec2};

use crate::{
    helper::{plot_arc, HEIGHT, WIDTH},
    ui::dda_line,
    Drawable,
};

const TIME_DELAY: f64 = 0.5;


#[derive(Debug, Clone)]
pub struct Particle {
    pub position: DVec2,
    pub velocity: DVec2,
    pub mass: f64,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub boundary: Boundary,
    pub center_of_mass: DVec2,
    pub total_mass: f64,
    pub particles: Vec<Particle>,
    pub divided: bool,
    pub northeast: Option<Box<Node>>,
    pub northwest: Option<Box<Node>>,
    pub southeast: Option<Box<Node>>,
    pub southwest: Option<Box<Node>>,
}

#[derive(Debug, Clone)]
pub struct Boundary {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}


impl Boundary {
    pub fn contains(&self, point: &DVec2) -> bool {
        point.x >= self.x - self.width &&
        point.x < self.x + self.width &&
        point.y >= self.y - self.height &&
        point.y < self.y + self.height
    }

    pub fn expand_to_fit(&mut self, point: &DVec2) {
        while !self.contains(point) {
            self.width *= 2.0;
            self.height *= 2.0;
            self.x = 0.0;
            self.y = 0.0;
        }
    }
}

impl Node {
    pub fn new(boundary: Boundary) -> Self {
        Node {
            boundary,
            center_of_mass: dvec2(0.0, 0.0),
            total_mass: 0.0,
            particles: Vec::new(),
            divided: false,
            northeast: None,
            northwest: None,
            southeast: None,
            southwest: None,
        }
    }

    pub fn insert(&mut self, particle: Particle) {
        if !self.boundary.contains(&particle.position) {
            return;
        }

        self.total_mass += particle.mass;
        self.center_of_mass.x = (self.center_of_mass.x * (self.total_mass - particle.mass) + particle.position.x * particle.mass) / self.total_mass;
        self.center_of_mass.y = (self.center_of_mass.y * (self.total_mass - particle.mass) + particle.position.y * particle.mass) / self.total_mass;

        if self.particles.len() < 4 {
            self.particles.push(particle);
            return;
        }

        if !self.divided {
            self.subdivide();
        }

        self.northeast.as_mut().unwrap().insert(particle.clone());
        self.northwest.as_mut().unwrap().insert(particle.clone());
        self.southeast.as_mut().unwrap().insert(particle.clone());
        self.southwest.as_mut().unwrap().insert(particle);
    }

    pub fn subdivide(&mut self) {
        let x = self.boundary.x;
        let y = self.boundary.y;
        let w = self.boundary.width / 2.0;
        let h = self.boundary.height / 2.0;

        self.northeast = Some(Box::new(Node::new(Boundary { x: x + w, y: y - h, width: w, height: h })));
        self.northwest = Some(Box::new(Node::new(Boundary { x: x - w, y: y - h, width: w, height: h })));
        self.southeast = Some(Box::new(Node::new(Boundary { x: x + w, y: y + h, width: w, height: h })));
        self.southwest = Some(Box::new(Node::new(Boundary { x: x - w, y: y + h, width: w, height: h })));
        
        self.divided = true;
    }

    pub fn compute_force(&self, particle: &Particle) -> DVec2 {
        const G: f64 = 6.67e-11;

        let dx = self.center_of_mass.x - particle.position.x;
        let dy = self.center_of_mass.y - particle.position.y;
        let dist_sq = dx * dx + dy * dy + 1e-6; // Softening factor to avoid singularities
        let force_mag = G * self.total_mass * particle.mass / dist_sq;
        let dist = dist_sq.sqrt();
        
        if dist > 0.0 {
            DVec2 { x: force_mag * dx / dist, y: force_mag * dy / dist }
        } else {
            DVec2 { x: 0.0, y: 0.0 }
        }
    }
}

fn update_particles(particles: &mut Vec<Particle>, root: &Node, dt: f64) {
    for particle in particles.iter_mut() {
        let force = root.compute_force(particle);
        let ax = force.x / particle.mass;
        let ay = force.y / particle.mass;
        
        particle.velocity.x += ax * dt;
        particle.velocity.y += ay * dt;
        
        particle.position.x += particle.velocity.x * dt;
        particle.position.y += particle.velocity.y * dt;
    }
}



#[derive(Clone)]
pub struct World {
    pub root: Node
}

impl World {
    // pub fn get_id_offset(&self) -> usize {
    //     return get_current_id() - self.objects.len();
    // }
    pub fn update(&mut self) {
        
        
        fn update_particles_node(node: &mut Node) {
            let mut dt = 1.;

            if let Some(ref mut northeast) = node.northeast {
                update_particles_node(northeast);
            }
            if let Some(ref mut northwest) = node.northwest {
                update_particles_node(northwest);
            }
            if let Some(ref mut southeast) = node.southeast {
                update_particles_node(southeast);
            }
            if let Some(ref mut southwest) = node.southwest {
                update_particles_node(southwest);
            }

            let mut particles = node.particles.clone();
            update_particles(&mut particles, node, dt);
        }

        for _ in 0..1 {
            for particle in &self.root.particles {
                self.root.boundary.expand_to_fit(&particle.position);
            }
            
            let mut root = Node::new(self.root.boundary.clone());
            for particle in &self.root.particles {
                root.insert(particle.clone());
            }
            
            update_particles_node(&mut self.root);
            // println!("Updated particles: {:#?}", self.root);
        }

        // let secondaries = self.objects.clone();

        // Euler Integration
        // // For each object in the world
        // for obj in self.objects.iter_mut() {
        //     let mut net_force = dvec2(0., 0.);

        //     // print!("p{}: ", obj.id);

        //     for secondary in secondaries.clone().into_iter() {
        //         // Calculate the force of gravity from all other objects
        //         if secondary.id == obj.id {
        //             continue;
        //         } // If it's not you.
        //         // print!("{}, ", secondary.id);

        //         let grav = obj.gravity(&secondary);

        //         // Forces are directional
        //         let directional_vector = (secondary.position - obj.position).normalize() * grav;
        //         net_force += directional_vector;
        //     }
        //     obj.force = net_force;
        //     // print!("\n");

        // }

        // for obj in self.objects.iter_mut() {
        //     // Apply the timestep and Newton's Second
        //     obj.velocity += (obj.force / obj.mass) * TIME_DELAY;
        //     obj.position += obj.velocity * TIME_DELAY;
        // }
    }
}

impl Drawable for Vec<Particle> {
    fn draw(&self, buffer: &mut [u32], zoom: f32, _: f32, center: (f64, f64)) {
        for particle in self.clone() {
            let au_factor: f64 = 1.496e8 / zoom as f64;
            const CENTER: DVec2 = dvec2((WIDTH as f64) / 2., HEIGHT as f64 / 2.);

            let x = (particle.position.x.floor() / au_factor) + CENTER.x + center.0;
            let y = (particle.position.y.floor() / au_factor) + CENTER.y + center.1;

            plot_arc(buffer, x, y, 5., 0xfffffff, true, true, true, true);
        }
    }
}

impl Drawable for World {
    fn draw(&self, buffer: &mut [u32], zoom: f32, vectors: f32, center: (f64, f64)) {
        // self.objects.clone().into_iter().for_each(|obj| {
        //     obj.draw(buffer, zoom, 0., center);
        // });


        fn draw_particles(node: Box<Node>, buffer: &mut [u32], zoom: f32, vectors: f32, center: (f64, f64)) {
            if node.northeast.is_some() {
                draw_particles(node.northeast.unwrap(), buffer, zoom, vectors, center);
            }
            if node.northwest.is_some() {
                draw_particles(node.northwest.unwrap(), buffer, zoom, vectors, center);
            }
            if node.southeast.is_some() {
                draw_particles(node.southeast.unwrap(), buffer, zoom, vectors, center);
            }
            if node.southwest.is_some() {
                draw_particles(node.southwest.unwrap(), buffer, zoom, vectors, center);
            }

            node.particles.draw(buffer, zoom, vectors, center);
        }

        // self.root.particles.draw(buffer, zoom, vectors, center);
        draw_particles(Box::new(self.root.clone()), buffer, zoom, vectors, center);

        // if vectors % 3. == 0. {
        //     self.objects.clone().into_iter().for_each(|obj| {
        //         let au_factor: f64 = 1.496e8 / zoom as f64;
        //         const CENTER: DVec2 = dvec2((WIDTH as f64) / 2., HEIGHT as f64 / 2.);

        //         let x = (obj.position.x.floor() / au_factor) + CENTER.x + center.0;
        //         let y = (obj.position.y.floor() / au_factor) + CENTER.y + center.1;

        //         let factor = (obj.velocity / obj.force).normalize().abs().element_sum();

        //         dda_line(
        //             buffer,
        //             x,
        //             y,
        //             x + (obj.force.normalize() * obj.radius).x,
        //             y + (obj.force.normalize() * obj.radius).y,
        //             0xffff0000,
        //         ); // force

        //         dda_line(
        //             buffer,
        //             x,
        //             y,
        //             x + (obj.velocity.normalize() * obj.radius * factor).x,
        //             y + (obj.velocity.normalize() * obj.radius * factor).y,
        //             0xff00ff00,
        //         );
        //     });
        // }

        // if vectors % 5. == 0. {
        //     let mut point = PointWeight {
        //         id: 0,
        //         velocity: dvec2(0., 0.),
        //         force: dvec2(0., 0.),
        //         position: dvec2(0., 0.),
        //         mass: 50.,
        //         radius: 0.,
        //         color: 0x00000000,
        //     };

        //     let x_divs = 30;
        //     let y_divs = 20;
        //     for x_factor in (-(x_divs / 2))..(x_divs / 2) {
        //         for y_factor in (-(y_divs / 2))..(y_divs / 2) {
        //             let au_factor: f64 = 1.496e8 / zoom as f64;

        //             let x_factor = x_factor as f64;
        //             let y_factor = y_factor as f64;
        //             let x_divs = x_divs as f64;
        //             let y_divs = y_divs as f64;

        //             point.position = dvec2(
        //                 ((x_factor / x_divs) * (WIDTH as f64) * au_factor) + center.0,
        //                 ((y_factor / y_divs) * (HEIGHT as f64) * au_factor) + center.1,
        //             );

        //             let mut net_force = dvec2(0., 0.);

        //             for secondary in self.objects.clone().into_iter() {
        //                 let grav = point.gravity(&secondary);

        //                 // Forces are directional
        //                 let directional_vector =
        //                     (secondary.position - point.position).normalize() * grav;
        //                 net_force += directional_vector;
        //             }
        //             point.force = net_force;

        //             const CENTER: DVec2 = dvec2((WIDTH as f64) / 2., HEIGHT as f64 / 2.);

        //             let x = (point.position.x.floor() / au_factor) + CENTER.x + center.0;
        //             let y = (point.position.y.floor() / au_factor) + CENTER.y + center.1;

        //             // println!("{}m{}", x, y);

        //             dda_line(
        //                 buffer,
        //                 x,
        //                 y,
        //                 x + (point.force.normalize() * 20.).x,
        //                 y + (point.force.normalize() * 20.).y,
        //                 0x60606060,
        //             );
        //         }
        //     }
        // }
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

    pub fn new_with_vel(position: DVec2, vel: DVec2, mass: f64, mut color: u32, radius: f64) -> PointWeight {
        let id = id();

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
