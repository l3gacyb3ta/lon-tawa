use std::fs;
use std::io::{BufRead, BufReader, Read, Write};
use glam::DVec2;
use crate::gravity::PointWeight; // assumes PointWeight is defined in gravity.rs

// Save each object's state as a single line.
pub fn save_state(objects: &Vec<PointWeight>, path: &str, center: DVec2, zoom: f64) {
    let mut file = fs::File::create(path).expect("Unable to create save file");
    writeln!(file, "{} {} {}", center.x, center.y, zoom)
        .expect("Unable to write center and zoom to file");
    writeln!(file, "id pos.x pos.y vel.x vel.y mass radius force.x force.y color")
        .expect("Unable to write header to file");
    for obj in objects {
        // Write: id pos.x pos.y vel.x vel.y mass color radius force.x force.y
        writeln!(
            file,
            "{} {} {} {} {} {} {} {} {} {:x}",
            obj.id,
            obj.position.x, obj.position.y,
            obj.velocity.x, obj.velocity.y,
            obj.mass,
            obj.radius,
            obj.force.x, obj.force.y,
            obj.color,
        )
        .expect("Unable to write object to file");
    }
}

pub fn load_state(path: &str) -> (Vec<PointWeight>, DVec2, f64) {
    let file = fs::File::open(path).expect("Unable to open save file");
    let mut reader = BufReader::new(file);
    let mut objects = Vec::new();
    
    // Skip the header line
    let mut line = String::new();
    reader.read_line(&mut line).expect("Unable to read center position and zoom factor line");
    let fields: Vec<&str> = line.trim().split_whitespace().collect();
    let center_x: f64 = fields[0].parse().unwrap_or(0.);
    let center_y: f64 = fields[1].parse().unwrap_or(0.);

    reader.read_line(&mut line).expect("Unable to read header line");

    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        let fields: Vec<&str> = line.trim().split_whitespace().collect();
        if fields.len() < 9 {
            continue;
        }
        let id: usize = fields[0].parse().unwrap_or(0);
        let pos_x: f64 = fields[1].parse().unwrap_or(0.);
        let pos_y: f64 = fields[2].parse().unwrap_or(0.);
        let vel_x: f64 = fields[3].parse().unwrap_or(0.);
        let vel_y: f64 = fields[4].parse().unwrap_or(0.);
        let mass: f64 = fields[5].parse().unwrap_or(0.);
        let radius: f64 = fields[6].parse().unwrap_or(0.);
        let force_x: f64 = fields[7].parse().unwrap_or(0.);
        let force_y: f64 = fields[8].parse().unwrap_or(0.);
        let color: u32 = u32::from_str_radix(fields[9], 16).unwrap_or(0xffffffff);

        let mut obj = PointWeight::new_with_vel(
            DVec2::new(pos_x, pos_y),
            DVec2::new(vel_x, vel_y),
            mass,
            color,
            radius,
            id,
        );
        obj.force = DVec2::new(force_x, force_y);
        objects.push(obj);
    }
    
    (objects, DVec2::new(center_x, center_y), 1.0)
}
