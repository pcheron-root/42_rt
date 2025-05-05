use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct Position {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Debug, Deserialize)]
struct TrianglePoints {
    a: Position,
    b: Position,
    c: Position,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum ObjectType {
    Sphere { radius: f32 },
    Disk { radius: f32 },
    Cylinder { radius: f32, height: f32 },
    Tube { radius: f32, height: f32 },
    Cone { radius: f32, height: f32 },
    Cube { size: f32 },
    Plane {},
    Triangle { vertices: TrianglePoints },
}

#[derive(Debug, Deserialize)]
struct SceneObject {
    position: Position,
    #[serde(flatten)]
    object_type: ObjectType,
}

#[derive(Debug, Deserialize)]
struct World {
    objects: Vec<SceneObject>,
}

#[derive(Debug, Deserialize)]
struct Scene {
    world: World,
}

pub fn parse(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Read JSON file
    let json = fs::read_to_string(path)?;
    
    // Parse JSON into Rust data structures
    let root: Scene = serde_json::from_str(&json)?;

    // Process the objects
    for object in &root.world.objects {
        match &object.object_type {
            ObjectType::Sphere { radius } => {
                println!(
                    "Found Sphere with radius {} at position ({}, {}, {})",
                    radius, object.position.x, object.position.y, object.position.z
                );
            },
            ObjectType::Disk { radius } => {
                println!(
                    "Found Disk with radius {} at position ({}, {}, {})",
                    radius, object.position.x, object.position.y, object.position.z
                );
            },
            ObjectType::Cylinder { radius, height } => {
                println!(
                    "Found Cylinder with radius {} and height {} at position ({}, {}, {})",
                    radius, height, object.position.x, object.position.y, object.position.z
                );
            },
            ObjectType::Tube { radius, height } => {
                println!(
                    "Found Tube with radius {} and height {} at position ({}, {}, {})",
                    radius, height, object.position.x, object.position.y, object.position.z
                );
            },
            ObjectType::Cone { radius, height } => {
                println!(
                    "Found Cone with radius {} and height {} at position ({}, {}, {})",
                    radius, height, object.position.x, object.position.y, object.position.z
                );
            },
            ObjectType::Cube { size } => {
                println!(
                    "Found Cube with size {} at position ({}, {}, {})",
                    size, object.position.x, object.position.y, object.position.z
                );
            },
            ObjectType::Plane {} => {
                println!(
                    "Found Plane at position ({}, {}, {})",
                    object.position.x, object.position.y, object.position.z
                );
            },
            ObjectType::Triangle { vertices } => {
                println!(
                    "Found Triangle at position ({}, {}, {})",
                    object.position.x, object.position.y, object.position.z
                );
            }
        }
    }

    Ok(())
}