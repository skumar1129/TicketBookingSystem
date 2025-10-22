// Imports Serde traits for JSON serialization/deserialization.
use serde::{Deserialize, Serialize};
// Imports Vec for dynamic arrays, equivalent to C++'s std::vector.
use std::vec::Vec;
// Imports User and Train structs from sibling modules.
use super::user::User;
use super::train::Train;

// Defines the Vehicle struct, equivalent to the C++ Vehicle struct in vehicle.hpp.
// Derives Serialize, Deserialize, Clone, and Debug for JSON, copying, and printing.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Vehicle {
    pub vehicle_id: String,       // Stores the vehicle's unique identifier, equivalent to C++'s vehicleId: string.
    pub name: String,             // Stores the vehicle's name, equivalent to C++'s name: string.
    pub source: String,           // Stores the vehicle's starting point, equivalent to C++'s source: string.
    pub destination: String,      // Stores the vehicle's destination, equivalent to C++'s destination: string.
    pub time: i64,               // Stores the vehicle's timestamp, equivalent to C++'s time_t (mapped to i64).
    pub seats: Vec<Vec<User>>,   // Stores a 2D vector of Users for seating, equivalent to C++'s vector<vector<User>>.
}

// Defines a function to convert a Vehicle to a Train, equivalent to C++'s inline Train convert(Vehicle).
pub fn convert(vehicle: Vehicle) -> Train {
    Train {
        train_id: vehicle.vehicle_id,  // Copies vehicle_id to train_id, matching C++ field mapping.
        name: vehicle.name,           // Copies name field.
        source: vehicle.source,       // Copies source field.
        destination: vehicle.destination, // Copies destination field.
        time: vehicle.time,           // Copies time field.
        seats: vehicle.seats,         // Copies seats field (same type: Vec<Vec<User>>).
    }
}