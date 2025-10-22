// Imports Serde traits for JSON serialization/deserialization.
use serde::{Deserialize, Serialize};

// Defines the User struct, equivalent to the C++ User struct in user.hpp.
// Derives Serialize and Deserialize for JSON compatibility, Clone for copying, and Debug for printing.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub user_id: String,       // Stores the user's unique identifier, equivalent to C++'s userId: string.
    pub name: String,          // Stores the user's name, equivalent to C++'s name: string.
    pub aadhar_card: String,   // Stores the user's Aadhar card number, equivalent to C++'s aadharCard: string.
    // Note: The C++ User struct has a vector<Vehicle*> vehicles field, but it's unused in the provided code.
    // Omitted here to avoid complex ownership handling (e.g., Rc/Arc for pointers).
    // Could be added as: pub vehicles: Vec<Rc<Vehicle>> if needed, with proper imports.
}