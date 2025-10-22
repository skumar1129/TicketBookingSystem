// Imports io module for console input/output, equivalent to C++'s std::iostream.
use std::io::{self, Write};
// Imports User, Vehicle, and BookingService from the library.
use train_booking::entities::user::User;
use train_booking::entities::vehicle::Vehicle;
use train_booking::service::booking_service::{BookingService, BookingServiceImpl};

// Main function, returning io::Result for error handling.
fn main() -> io::Result<()> {
    let mut user_id = String::new(); // Initializes a string for user ID, equivalent to C++'s string userId.
    let mut name = String::new();    // Initializes a string for user name.
    let mut aadhar_card = String::new(); // Initializes a string for Aadhar card number.

    print!("Enter User ID: "); // Prompts for user ID, equivalent to C++'s cout.
    io::stdout().flush()?;     // Flushes stdout to ensure prompt is displayed before input.
    io::stdin().read_line(&mut user_id)?; // Reads a line of input, including spaces, unlike C++'s cin.
    user_id = user_id.trim().to_string(); // Trims whitespace and converts to owned String.

    print!("Enter Name: "); // Prompts for user name.
    io::stdout().flush()?;
    io::stdin().read_line(&mut name)?;
    name = name.trim().to_string(); // Trims and converts to String, allowing multi-word names.

    print!("Enter Aadhar Card Number: "); // Prompts for Aadhar card number.
    io::stdout().flush()?;
    io::stdin().read_line(&mut aadhar_card)?;
    aadhar_card = aadhar_card.trim().to_string(); // Trims and converts to String.

    println!("Enter the option: "); // Prompts for option selection.
    println!("Enter 1 to book the train"); // Displays available option, matching C++.
    let mut option = String::new(); // Initializes a string for the option input.
    io::stdin().read_line(&mut option)?; // Reads the option as a string.
    let option: i32 = option.trim().parse().unwrap_or(0); // Parses to i32, defaults to 0 if invalid.

    // Comments from C++ outlining intended flow, preserved here but not implemented.
    // signup login user using userid --> user.json
    // fetch train using train name --> train.json
    // booking --> fetch train using trainId, fetch seats, store user there, booking.json

    // Matches C++'s switch statement using Rust's match expression.
    match option {
        1 => { // Case for booking a train.
            let mut train_id = String::new(); // Initializes string for train ID.
            let mut train_name = String::new(); // Initializes string for train name.
            let mut source = String::new(); // Initializes string for source station.
            let mut destination = String::new(); // Initializes string for destination station.

            print!("Enter Train ID: "); // Prompts for train ID.
            io::stdout().flush()?;
            io::stdin().read_line(&mut train_id)?;
            train_id = train_id.trim().to_string(); // Trims and converts to String.

            print!("Enter Train Name: "); // Prompts for train name.
            io::stdout().flush()?;
            io::stdin().read_line(&mut train_name)?;
            train_name = train_name.trim().to_string();

            print!("Enter Source Station: "); // Prompts for source station.
            io::stdout().flush()?;
            io::stdin().read_line(&mut source)?;
            source = source.trim().to_string();

            print!("Enter Destination Station: "); // Prompts for destination station.
            io::stdout().flush()?;
            io::stdin().read_line(&mut destination)?;
            destination = destination.trim().to_string();

            // Creates a User struct with input data, equivalent to C++'s User initialization.
            let user = User {
                user_id,
                name,
                aadhar_card,
            };

            // Creates a BookingService for Vehicle, matching C++'s use of BookingService<Vehicle>.
            let booking_service = BookingServiceImpl::<Vehicle>::new();
            // Books the train (as a Vehicle), propagating any file I/O errors.
            booking_service.book(train_id, user, train_name, source, destination)?;

            println!("Train booked successfully!"); // Outputs success message.
        }
        _ => println!("Invalid option"), // Default case for invalid input, matching C++.
    }

    Ok(()) // Returns success for the main function.
}