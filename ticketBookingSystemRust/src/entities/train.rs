// Imports Serde traits for JSON serialization/deserialization.
use serde::{Deserialize, Serialize};
// Imports Vec for dynamic arrays, equivalent to C++'s std::vector.
use std::vec::Vec;
// Imports the User struct from the user module.
use super::user::User;

// Defines the Train struct, equivalent to the C++ Train struct in train.hpp.
// Derives Serialize, Deserialize, Clone, and Debug for JSON, copying, and printing.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Train {
    pub train_id: String,         // Stores the train's unique identifier, equivalent to C++'s trainId: string.
    pub name: String,             // Stores the train's name, equivalent to C++'s name: string.
    pub source: String,           // Stores the train's starting station, equivalent to C++'s source: string.
    pub destination: String,      // Stores the train's destination station, equivalent to C++'s destination: string.
    pub time: i64,               // Stores the train's timestamp, equivalent to C++'s time_t (mapped to i64 for Unix timestamps).
    pub seats: Vec<Vec<User>>,   // Stores a 2D vector of Users for seating, equivalent to C++'s vector<vector<User>>.
}