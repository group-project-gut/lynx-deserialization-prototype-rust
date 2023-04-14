use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
struct Scene {
    entities: Vec<Entity>
}

#[derive(Debug, Serialize, Deserialize)]
struct Entity {
    r#type: String,
    attributes: HashMap<String, Value>
}

fn main() {
    let serialized_scene = "{\"entities\": [{\"type\": \"Object\", \"attributes\": {\"id\": 123, \"name\": \"dummy\", \"position\": {\"x\": 0, \"y\": 0}, \"additional_positions\": [], \"state\": \"\", \"walkable\": false, \"tick\": \"\", \"on_death\": \"\", \"owner\": \"\"}}, {\"type\": \"Move\", \"attributes\": {\"object_id\": 456, \"movement\": {\"x\": 1, \"y\": 1}}}]}";
    let scene: Scene = serde_json::from_str(&serialized_scene).unwrap();
    println!("{:?}", scene.entities);
    assert!(scene.entities[0].attributes["id"] == 123);
    assert!(scene.entities[1].attributes["object_id"] == 456);
}
