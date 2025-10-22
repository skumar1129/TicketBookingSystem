// Imports SystemTime and UNIX_EPOCH for timestamp handling, equivalent to C++'s time(nullptr).
use std::time::{SystemTime, UNIX_EPOCH};
// Imports User, Vehicle, and Train structs from the entities module.
use super::super::entities::user::User;
use super::super::entities::vehicle::Vehicle;
use super::super::entities::train::Train;
// Imports FileIO trait and FileIOImpl for file operations.
use super::file_io::{FileIO, FileIOImpl};

// Defines a BookingService trait, replacing C++'s BookingService template class.
pub trait BookingService {
    // Declares a method to book an entity, returning io::Result for error handling.
    fn book(&self, entity_id: String, user: User, name: String, source: String, destination: String) -> std::io::Result<()>;
    // Declares a method to cancel a booking (no return, as in C++).
    fn cancel_booking(&self, entity_id: String, user_id: String);
    // Declares a method to print booking details (no return, as in C++).
    fn print_booking(&self, entity_id: String, user_id: String);
}

// Defines a generic BookingServiceImpl struct, similar to C++'s BookingService class.
pub struct BookingServiceImpl<T> {
    _phantom: std::marker::PhantomData<T>, // Ensures type safety for generic T.
}

// Implements BookingServiceImpl for any T, providing a constructor and save_booking.
impl<T> BookingServiceImpl<T> {
    // Creates a new BookingServiceImpl instance.
    pub fn new() -> Self {
        BookingServiceImpl {
            _phantom: std::marker::PhantomData, // Initializes PhantomData for type T.
        }
    }

    // Saves an entity using FileIO, equivalent to C++'s saveBooking private method.
    fn save_booking(&self, entity: &T) -> std::io::Result<()>
    where
        FileIOImpl<T>: super::file_io::FileIO<Item = T> // Constrains the FileIO implementation for T to implement FileIO with Item = T.
    {
        let file = FileIOImpl::<T>::new(); // Creates a FileIO instance for type T.
        file.save_to_file(entity) // Delegates to FileIO's save_to_file.
    }
}

// Implements BookingService for Vehicle, equivalent to C++'s BookingService<Vehicle>.
impl BookingService for BookingServiceImpl<Vehicle> {
    // Books a Vehicle, equivalent to C++'s book method.
    fn book(&self, entity_id: String, user: User, name: String, source: String, destination: String) -> std::io::Result<()> {
        // Creates a new Vehicle with provided details.
        let vehicle = Vehicle {
            vehicle_id: entity_id, // Sets vehicle_id from input, matching C++.
            name,                 // Sets name from input.
            source,               // Sets source from input.
            destination,          // Sets destination from input.
            time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64, // Sets current timestamp, equivalent to time(nullptr).
            seats: vec![vec![user]], // Initializes seats with one row containing the user, matching C++'s {{user}}.
        };
        self.save_booking(&vehicle) // Saves the Vehicle to db.json.
    }

    // Placeholder for canceling a booking, matching C++'s empty implementation.
    fn cancel_booking(&self, _entity_id: String, _user_id: String) {
        println!("Cancel booking not implemented"); // Prints a placeholder message.
    }

    // Placeholder for printing a booking, matching C++'s empty implementation.
    fn print_booking(&self, _entity_id: String, _user_id: String) {
        println!("Print booking not implemented"); // Prints a placeholder message.
    }
}

// Implements BookingService for Train, equivalent to C++'s BookingService<Train>.
impl BookingService for BookingServiceImpl<Train> {
    // Books a Train, equivalent to C++'s book method.
    fn book(&self, entity_id: String, user: User, name: String, source: String, destination: String) -> std::io::Result<()> {
        // Creates a new Train with provided details.
        let train = Train {
            train_id: entity_id, // Sets train_id from input, matching C++.
            name,
            source,
            destination,
            time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64, // Sets current timestamp.
            seats: vec![vec![user]], // Initializes seats with one row containing the user.
        };
        self.save_booking(&train) // Saves the Train to db.json.
    }

    // Placeholder for canceling a booking.
    fn cancel_booking(&self, _entity_id: String, _user_id: String) {
        println!("Cancel booking not implemented");
    }

    // Placeholder for printing a booking.
    fn print_booking(&self, _entity_id: String, _user_id: String) {
        println!("Print booking not implemented");
    }
}