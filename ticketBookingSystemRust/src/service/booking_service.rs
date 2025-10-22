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

    // Cancels a booking for a Vehicle by user_id. Reads vehicles from storage,
    // removes the user from the seats for the matched vehicle and saves the updated vehicle.
    fn cancel_booking(&self, entity_id: String, user_id: String)
    where
        FileIOImpl<Vehicle>: super::file_io::FileIO<Item = Vehicle>,
    {
        let file = FileIOImpl::<Vehicle>::new();
        match file.read_from_file() {
            Ok(mut vehicles) => {
                if let Some(vehicle) = vehicles.iter_mut().find(|v| v.vehicle_id == entity_id) {
                    let mut removed = false;
                    for row in vehicle.seats.iter_mut() {
                        let before = row.len();
                        row.retain(|u| u.user_id != user_id);
                        if row.len() < before {
                            removed = true;
                        }
                    }
                    // Remove empty rows
                    vehicle.seats.retain(|r| !r.is_empty());

                    if removed {
                        if let Err(e) = self.save_booking(vehicle) {
                            eprintln!("Failed to save updated booking: {}", e);
                        } else {
                            println!("Cancelled booking for user {}", user_id);
                        }
                    } else {
                        println!("No booking found for user {} on vehicle {}", user_id, entity_id);
                    }
                } else {
                    println!("Vehicle with id {} not found", entity_id);
                }
            }
            Err(e) => eprintln!("Failed to read vehicles: {}", e),
        }
    }

    // Prints booking details for a given vehicle and user.
    fn print_booking(&self, entity_id: String, user_id: String)
    where
        FileIOImpl<Vehicle>: super::file_io::FileIO<Item = Vehicle>,
    {
        let file = FileIOImpl::<Vehicle>::new();
        match file.read_from_file() {
            Ok(vehicles) => {
                if let Some(vehicle) = vehicles.iter().find(|v| v.vehicle_id == entity_id) {
                    println!(
                        "Vehicle: {} (id: {}), {} -> {}, time: {}",
                        vehicle.name, vehicle.vehicle_id, vehicle.source, vehicle.destination, vehicle.time
                    );
                    let mut found = false;
                    for (row_i, row) in vehicle.seats.iter().enumerate() {
                        for (col_i, user) in row.iter().enumerate() {
                            if user.user_id == user_id {
                                println!(
                                    "Found booking - row: {}, col: {}, user: {} (id: {})",
                                    row_i, col_i, user.name, user.user_id
                                );
                                found = true;
                            }
                        }
                    }
                    if !found {
                        println!("No booking found for user {} on vehicle {}", user_id, entity_id);
                    }
                } else {
                    println!("Vehicle with id {} not found", entity_id);
                }
            }
            Err(e) => eprintln!("Failed to read vehicles: {}", e),
        }
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

    fn cancel_booking(&self, entity_id: String, user_id: String)
    where
        FileIOImpl<Train>: super::file_io::FileIO<Item = Train>,
    {
        let file = FileIOImpl::<Train>::new();
        match file.read_from_file() {
            Ok(mut trains) => {
                if let Some(train) = trains.iter_mut().find(|t| t.train_id == entity_id) {
                    let mut removed = false;
                    for row in train.seats.iter_mut() {
                        let before = row.len();
                        row.retain(|u| u.user_id != user_id);
                        if row.len() < before {
                            removed = true;
                        }
                    }
                    // Remove empty rows
                    train.seats.retain(|r| !r.is_empty());

                    if removed {
                        if let Err(e) = self.save_booking(train) {
                            eprintln!("Failed to save updated booking: {}", e);
                        } else {
                            println!("Cancelled booking for user {}", user_id);
                        }
                    } else {
                        println!("No booking found for user {} on train {}", user_id, entity_id);
                    }
                } else {
                    println!("Train with id {} not found", entity_id);
                }
            }
            Err(e) => eprintln!("Failed to read trains: {}", e),
        }
    }

    fn print_booking(&self, entity_id: String, user_id: String)
    where
        FileIOImpl<Train>: super::file_io::FileIO<Item = Train>,
    {
        let file = FileIOImpl::<Train>::new();
        match file.read_from_file() {
            Ok(trains) => {
                if let Some(train) = trains.iter().find(|t| t.train_id == entity_id) {
                    println!(
                        "Train: {} (id: {}), {} -> {}, time: {}",
                        train.name, train.train_id, train.source, train.destination, train.time
                    );
                    let mut found = false;
                    for (row_i, row) in train.seats.iter().enumerate() {
                        for (col_i, user) in row.iter().enumerate() {
                            if user.user_id == user_id {
                                println!(
                                    "Found booking - row: {}, col: {}, user: {} (id: {})",
                                    row_i, col_i, user.name, user.user_id
                                );
                                found = true;
                            }
                        }
                    }
                    if !found {
                        println!("No booking found for user {} on train {}", user_id, entity_id);
                    }
                } else {
                    println!("Train with id {} not found", entity_id);
                }
            }
            Err(e) => eprintln!("Failed to read trains: {}", e),
        }
    }
}