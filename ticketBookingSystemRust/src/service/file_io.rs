// Imports File and OpenOptions for file operations, equivalent to C++'s std::fstream.
use std::fs::{File, OpenOptions};
// Imports Read and Write for file I/O, equivalent to C++'s file stream operations.
use std::io::{Read, Write};
// Imports serde_json for JSON handling, equivalent to C++'s nlohmann::json.
use serde_json::{Value, json};
// Imports Vehicle, Train, and User structs from the entities module.
use super::super::entities::vehicle::Vehicle;
use super::super::entities::train::Train;
use super::super::entities::user::User;

// Defines a FileIO trait to abstract file operations, replacing C++'s FileIO template class.
pub trait FileIO {
    type Item;
    // Declares a method to save an entity to a file, returning io::Result for error handling.
    fn save_to_file(&self, entity: &Self::Item) -> std::io::Result<()>;
    // Declares a method to read entities from a file, returning a vector wrapped in io::Result.
    fn read_from_file(&self) -> std::io::Result<Vec<Self::Item>>;
}

// Defines a generic FileIOImpl struct to hold file I/O configuration, similar to C++'s FileIO class.
pub struct FileIOImpl<T> {
    filename: String,                  // Stores the filename ("db.json"), equivalent to C++'s const string filename.
    _phantom: std::marker::PhantomData<T>, // PhantomData ensures type safety for the generic T, unused at runtime.
}

// Implements FileIOImpl for any T, providing a constructor.
impl<T> FileIOImpl<T> {
    // Creates a new FileIOImpl instance with default filename "db.json".
    pub fn new() -> Self {
        FileIOImpl {
            filename: "db.json".to_string(), // Sets the filename, matching C++'s const string filename = "db.json".
            _phantom: std::marker::PhantomData, // Initializes PhantomData for type T.
        }
    }
}
// Implements FileIO trait for FileIOImpl<Vehicle>, equivalent to C++'s FileIO<Vehicle> specialization.
impl FileIO for FileIOImpl<Vehicle> {
    type Item = Vehicle;

    // Saves a Vehicle to db.json, equivalent to C++'s FileIO<Vehicle>::saveToFile.
    fn save_to_file(&self, entity: &Vehicle) -> std::io::Result<()> {
        // Initializes an empty JSON array to store data, equivalent to C++'s json j.
        let mut j = json!([]);
        // Attempts to open and read existing JSON data from the file.
        if let Ok(mut file) = File::open(&self.filename) {
            let mut contents = String::new(); // Buffer to store file contents.
            file.read_to_string(&mut contents)?; // Reads file into string, propagates errors.
            // Parses file contents as JSON, falling back to empty array if invalid.
            j = serde_json::from_str(&contents).unwrap_or(json!([]));
        }

        // Converts seats (Vec<Vec<User>>) to JSON format, matching C++'s seats serialization.
        let seats_json: Vec<Vec<serde_json::Value>> = entity.seats.iter().map(|row| {
            row.iter().map(|user| {
                // Creates a JSON object for each User, matching C++'s userJson structure.
                json!({
                    "userId": user.user_id,      // Serializes user_id field.
                    "name": user.name,           // Serializes name field.
                    "aadharCard": user.aadhar_card, // Serializes aadhar_card field.
                })
            }).collect() // Collects user JSON objects into a row array.
        }).collect(); // Collects row arrays into seats array.

        // Creates a JSON object for the Vehicle, matching C++'s entityJson.
        let entity_json = json!({
            "vehicleId": entity.vehicle_id,   // Serializes vehicle_id.
            "name": entity.name,             // Serializes name.
            "source": entity.source,         // Serializes source.
            "destination": entity.destination, // Serializes destination.
            "time": entity.time,             // Serializes time as i64.
            "seats": seats_json,             // Includes serialized seats.
        });

        // Appends the Vehicle JSON to the main array, matching C++'s j.push_back(entityJson).
        if let Some(arr) = j.as_array_mut() {
            arr.push(entity_json);
        }

        // Opens the file for writing, creating it if it doesn't exist, and truncating existing content.
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.filename)?; // Propagates file open errors.
        // Writes the JSON array to the file with pretty formatting (4-space indentation).
        file.write_all(serde_json::to_string_pretty(&j)?.as_bytes())?;
        Ok(()) // Returns success if writing completes.
    }

    // Reads Vehicles from db.json, equivalent to C++'s FileIO<Vehicle>::readFromFile.
    fn read_from_file(&self) -> Result<Vec<Vehicle>, std::io::Error> {
        let mut entities = Vec::new(); // Initializes an empty vector to store Vehicles.
        // Attempts to open the file, returning an empty vector if it doesn't exist.
        let mut file = match File::open(&self.filename) {
            Ok(file) => file,
            Err(_) => return Ok(entities), // Matches C++'s behavior of returning empty vector.
        };

        let mut contents = String::new(); // Buffer for file contents.
        file.read_to_string(&mut contents)?; // Reads file into string, propagates errors.
        // Parses file contents as JSON, falling back to empty array if invalid.
        let j: Value = serde_json::from_str(&contents).unwrap_or(json!([]));

        // Iterates over JSON array elements, matching C++'s for (const auto& item : j).
        if let Some(arr) = j.as_array() {
            for item in arr {
                // Creates a new Vehicle with default values, to be populated from JSON.
                let mut vehicle = Vehicle {
                    vehicle_id: item["vehicleId"].as_str().unwrap_or("").to_string(), // Deserializes vehicle_id, defaults to empty string.
                    name: item["name"].as_str().unwrap_or("").to_string(),           // Deserializes name.
                    source: item["source"].as_str().unwrap_or("").to_string(),       // Deserializes source.
                    destination: item["destination"].as_str().unwrap_or("").to_string(), // Deserializes destination.
                    time: item["time"].as_i64().unwrap_or(0),                        // Deserializes time, defaults to 0.
                    seats: Vec::new(),                                              // Initializes empty seats vector.
                };

                // Deserializes seats from JSON, matching C++'s seats conversion.
                if let Some(seats) = item["seats"].as_array() {
                    for row_json in seats {
                        let mut row = Vec::new(); // Initializes a row of Users.
                        if let Some(users) = row_json.as_array() {
                            for user_json in users {
                                // Creates a User from JSON, matching C++'s user deserialization.
                                let user = User {
                                    user_id: user_json["userId"].as_str().unwrap_or("").to_string(),
                                    name: user_json["name"].as_str().unwrap_or("").to_string(),
                                    aadhar_card: user_json["aadharCard"].as_str().unwrap_or("").to_string(),
                                };
                                row.push(user); // Adds User to the row.
                            }
                        }
                        vehicle.seats.push(row); // Adds row to seats.
                    }
                }
                entities.push(vehicle); // Adds Vehicle to entities vector.
            }
        }
        Ok(entities) // Returns the vector of deserialized Vehicles.
    }
}
// Implements FileIO trait for FileIOImpl<Train>, equivalent to C++'s FileIO<Train> specialization.
impl FileIO for FileIOImpl<Train> {
    type Item = Train;

    // Saves a Train to db.json, equivalent to C++'s FileIO<Train>::saveToFile.
    fn save_to_file(&self, entity: &Train) -> std::io::Result<()> {
        // Initializes an empty JSON array to store data.
        let mut j = json!([]);
        // Attempts to read existing JSON data from the file.
        if let Ok(mut file) = File::open(&self.filename) {
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            j = serde_json::from_str(&contents).unwrap_or(json!([]));
        }

        // Converts seats to JSON, identical to Vehicle's seats serialization.
        let seats_json: Vec<Vec<serde_json::Value>> = entity.seats.iter().map(|row| {
            row.iter().map(|user| {
                json!({
                    "userId": user.user_id,
                    "name": user.name,
                    "aadharCard": user.aadhar_card,
                })
            }).collect()
        }).collect();

        // Creates a JSON object for the Train, differing from Vehicle in the trainId key.
        let entity_json = json!({
            "trainId": entity.train_id,   // Uses train_id instead of vehicle_id.
            "name": entity.name,
            "source": entity.source,
            "destination": entity.destination,
            "time": entity.time,
            "seats": seats_json,
        });

        // Appends the Train JSON to the main array.
        if let Some(arr) = j.as_array_mut() {
            arr.push(entity_json);
        }

        // Writes the JSON array to the file with pretty formatting.
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.filename)?;
        file.write_all(serde_json::to_string_pretty(&j)?.as_bytes())?;
        Ok(())
    }

    // Reads Trains from db.json, equivalent to C++'s FileIO<Train>::readFromFile.
    fn read_from_file(&self) -> Result<Vec<Train>, std::io::Error> {
        let mut entities = Vec::new();
        let mut file = match File::open(&self.filename) {
            Ok(file) => file,
            Err(_) => return Ok(entities), // Returns empty vector if file doesn't exist.
        };

        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let j: Value = serde_json::from_str(&contents).unwrap_or(json!([]));

        if let Some(arr) = j.as_array() {
            for item in arr {
                let mut train = Train {
                    train_id: item["trainId"].as_str().unwrap_or("").to_string(), // Uses trainId key.
                    name: item["name"].as_str().unwrap_or("").to_string(),
                    source: item["source"].as_str().unwrap_or("").to_string(),
                    destination: item["destination"].as_str().unwrap_or("").to_string(),
                    time: item["time"].as_i64().unwrap_or(0),
                    seats: Vec::new(),
                };

                // Deserializes seats, identical to Vehicle's seats deserialization.
                if let Some(seats) = item["seats"].as_array() {
                    for row_json in seats {
                        let mut row = Vec::new();
                        if let Some(users) = row_json.as_array() {
                            for user_json in users {
                                let user = User {
                                    user_id: user_json["userId"].as_str().unwrap_or("").to_string(),
                                    name: user_json["name"].as_str().unwrap_or("").to_string(),
                                    aadhar_card: user_json["aadharCard"].as_str().unwrap_or("").to_string(),
                                };
                                row.push(user);
                            }
                        }
                        train.seats.push(row);
                    }
                }
                entities.push(train);
            }
        }
        Ok(entities)
    }
}