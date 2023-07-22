// Import required libraries
#[macro_use]
extern crate rocket;

mod ai_logic;

// Import Rocket and other required modules
use rocket::State;
use ai_logic::get_chat_response;

#[get("/chat/<input>")]
fn chat(input: String, ai_state: &State<AIState>) -> String {
    let response = get_chat_response(&input);
    response
}

#[launch]
fn rocket() -> _ {
    let ai_state = AIState {}; // If you need to store AI state across requests
    rocket::build().mount("/", routes![chat]).manage(ai_state)
}
