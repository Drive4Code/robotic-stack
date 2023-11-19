use std::vec;
// Robotics lib imports
use robotics_lib::energy::Energy;
use robotics_lib::runner::Robot;
use robotics_lib::runner::Runnable;
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
use robotics_lib::world::environmental_conditions::WeatherType;
use robotics_lib::world::tile::TileType;
use robotics_lib::world::worldgenerator::Generator;
use robotics_lib::world::worldgenerator::check_world;
use robotics_lib::{interface::plot, world::World, world::tile::Tile, world::tile::Content};


// Rocket Imports
use rocket::serde::json::{json, Value};

#[macro_use]
extern crate robotics_lib;
#[macro_use]
extern crate rocket;

#[get("/test/<action>")]
fn test(action: &str) -> String {
    format!("AYO IMA {}", action)
}

#[get("/world?<prev>")]
fn world(prev:Option<String>) -> Value {
    // let generator = "we gottas implement this"
    // let world:World = World::new(generator);
    let robot = Robot::new();
    let tile1 = Tile {
        tile_type: TileType::Grass,
        content: Content::Fire,
    };
    let tile2 = Tile {
        tile_type: TileType::Street,
        content: Content::Garbage(1),
    };
    let map = vec![
        vec![tile2.clone(), tile1.clone(), tile1.clone(), tile1.clone()],
        vec![tile1.clone(), tile2.clone(), tile1.clone(), tile1.clone()],
        vec![tile1.clone(), tile1.clone(), tile2.clone(), tile1.clone()],
        vec![tile1.clone(), tile1.clone(), tile1.clone(), tile2.clone()],
    ];
    let conditions = EnvironmentalConditions::new(&[WeatherType::Sunny, WeatherType::Rainy], 15, 20);
    let wrld = World::new(
        map.to_owned(),
        conditions,
    );
    println!("{:?}, check: {:?}", wrld, check_world(&map));
    let plotted = plot(&wrld);  //DONT WORK
    println!("{:?}", plotted);
    // format!("{} {} {} \n")
    let mut result:Vec<String> = vec![];
    for i in plotted {
        for j in i {
            result.push(format!("{:?}", j.unwrap_or(Tile { tile_type: TileType::Grass, content: Content::None })));
        }
    }
    json![result]
    // result.get(1).unwrap().to_owned()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![test, world])
}
