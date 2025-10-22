// Declares the entities module, containing user, train, and vehicle definitions.
pub mod entities {
    pub mod user;    // Includes the user module (equivalent to user.hpp).
    pub mod train;   // Includes the train module (equivalent to train.hpp).
    pub mod vehicle; // Includes the vehicle module (equivalent to vehicle.hpp).
}

// Declares the service module, containing file I/O and booking service logic.
pub mod service {
    pub mod file_io;        // Includes the file_io module (equivalent to fileIO.hpp/cpp).
    pub mod booking_service; // Includes the booking_service module (equivalent to bookingService.hpp/cpp).
}