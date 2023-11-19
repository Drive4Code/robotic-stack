# [macro_use] extern crate robotics_lib;
# [macro_use] extern crate rocket;

# [get("/test/<action>")]
fn test(action : &str) -> String {
    format!("AYo IMA {}", action)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![test])
}
