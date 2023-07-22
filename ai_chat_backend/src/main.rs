// Import required libraries
#[macro_use]
extern crate rocket;

mod ai_logic;

// Import Rocket and other required modules
use rocket::State;
use ai_logic::get_chat_response;

// Define the AIState struct to store AI state
struct AIState {
    // Add fields here to store AI state data
    greeting_message: String,
    user_name: Option<String>,
    // Add more fields as needed to store AI state data
}

#[get("/chat/<input>")]
fn chat(input: String, ai_state: &State<AIState>) -> String {
    let response = get_chat_response(&input);
    response
}

#[launch]
fn rocket() -> _ {
    let ai_state = AIState {
        greeting_message: "Hello! How can I assist you today?".to_string(),
        user_name: None, // This will be updated as the user interacts with the AI
        // Add more field initializations as needed
    };

    rocket::build().mount("/", routes![chat]).manage(ai_state)
}