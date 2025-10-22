// This file can be deleted as all implementations are now in the header file
// Indicates that this source file may be redundant, as the implementations have been moved to the header file (likely for inline definitions).

#include "fileIO.hpp"  // Includes the fileIO.hpp header file, which declares the FileIO template class and its specializations for Vehicle and Train.

// Template implementation for Vehicle
template <>  // Explicit specialization of the FileIO class for Vehicle type.
void FileIO<Vehicle>::saveToFile(Vehicle entity) {  // Defines the saveToFile method to serialize a Vehicle object to a JSON file.
    json j;  // Creates a JSON object (from nlohmann::json) to store the data.
    ifstream file(filename);  // Opens the input file (db.json, as defined in FileIO) for reading existing data.

    if (file.is_open()) {  // Checks if the file was successfully opened.
        file >> j;  // Reads the existing JSON data from the file into the json object.
        file.close();  // Closes the input file.
    }
    
    // Convert seats to a serializable format
    json seatsJson = json::array();  // Creates a JSON array to store the 2D seats structure.
    for (const auto& row : entity.seats) {  // Iterates over each row in the Vehicle's seats (vector<vector<User>>).
        json rowJson = json::array();  // Creates a JSON array for the current row.
        for (const auto& user : row) {  // Iterates over each User in the row.
            json userJson = {  // Creates a JSON object for the User with key-value pairs.
                {"userId", user.userId},  // Stores the user's ID.
                {"name", user.name},      // Stores the user's name.
                {"aadharCard", user.aadharCard}  // Stores the user's Aadhar card number.
            };
            rowJson.push_back(userJson);  // Adds the user JSON object to the row array.
        }
        seatsJson.push_back(rowJson);  // Adds the row JSON array to the seats array.
    }
    
    json entityJson = {  // Creates a JSON object for the Vehicle entity.
        {"vehicleId", entity.vehicleId},  // Stores the vehicle's ID.
        {"name", entity.name},            // Stores the vehicle's name.
        {"source", entity.source},        // Stores the vehicle's source.
        {"destination", entity.destination},  // Stores the vehicle's destination.
        {"time", entity.time},            // Stores the vehicle's timestamp.
        {"seats", seatsJson}              // Stores the serialized seats array.
    };
    
    j.push_back(entityJson);  // Appends the Vehicle JSON object to the main JSON array.
    
    ofstream outFile(filename);  // Opens the output file (db.json) for writing.
    outFile << j.dump(4);  // Writes the JSON data to the file with 4-space indentation for readability.
    outFile.close();  // Closes the output file.
}

template <>  // Explicit specialization of the FileIO class for Vehicle type.
vector<Vehicle> FileIO<Vehicle>::readFromFile() {  // Defines the readFromFile method to deserialize Vehicle objects from a JSON file.
    vector<Vehicle> entities;  // Creates a vector to store the deserialized Vehicle objects.
    ifstream file(filename);  // Opens the input file (db.json) for reading.
    
    if (!file.is_open()) {  // Checks if the file failed to open.
        return entities;  // Returns an empty vector if the file cannot be opened.
    }
    
    json j;  // Creates a JSON object to store the file's data.
    file >> j;  // Reads the JSON data from the file into the json object.
    file.close();  // Closes the input file.
    
    for (const auto& item : j) {  // Iterates over each JSON object in the JSON array.
        Vehicle entity;  // Creates a new Vehicle object to populate.
        entity.vehicleId = item["vehicleId"];  // Sets the vehicleId from the JSON object.
        entity.name = item["name"];  // Sets the name from the JSON object.
        entity.source = item["source"];  // Sets the source from the JSON object.
        entity.destination = item["destination"];  // Sets the destination from the JSON object.
        entity.time = item["time"];  // Sets the timestamp from the JSON object.
        
        // Convert seats back from JSON
        entity.seats.clear();  // Clears the seats vector to ensure it's empty before populating.
        for (const auto& rowJson : item["seats"]) {  // Iterates over each row in the seats JSON array.
            vector<User> row;  // Creates a vector for the current row of Users.
            for (const auto& userJson : rowJson) {  // Iterates over each user in the row JSON array.
                User user;  // Creates a new User object.
                user.userId = userJson["userId"];  // Sets the userId from the JSON object.
                user.name = userJson["name"];  // Sets the name from the JSON object.
                user.aadharCard = userJson["aadharCard"];  // Sets the Aadhar card number from the JSON object.
                row.push_back(user);  // Adds the User to the row vector.
            }
            entity.seats.push_back(row);  // Adds the row to the Vehicle's seats vector.
        }
        
        entities.push_back(entity);  // Adds the populated Vehicle to the entities vector.
    }
    
    return entities;  // Returns the vector of deserialized Vehicle objects.
}

// Template implementation for Train
template <>  // Explicit specialization of the FileIO class for Train type.
void FileIO<Train>::saveToFile(Train entity) {  // Defines the saveToFile method to serialize a Train object to a JSON file.
    json j;  // Creates a JSON object to store the data.
    ifstream file(filename);  // Opens the input file (db.json) for reading existing data.
    
    if (file.is_open()) {  // Checks if the file was successfully opened.
        file >> j;  // Reads the existing JSON data from the file into the json object.
        file.close();  // Closes the input file.
    }
    
    // Convert seats to a serializable format
    json seatsJson = json::array();  // Creates a JSON array to store the 2D seats structure.
    for (const auto& row : entity.seats) {  // Iterates over each row in the Train's seats (vector<vector<User>>).
        json rowJson = json::array();  // Creates a JSON array for the current row.
        for (const auto& user : row) {  // Iterates over each User in the row.
            json userJson = {  // Creates a JSON object for the User with key-value pairs.
                {"userId", user.userId},  // Stores the user's ID.
                {"name", user.name},      // Stores the user's name.
                {"aadharCard", user.aadharCard}  // Stores the user's Aadhar card number.
            };
            rowJson.push_back(userJson);  // Adds the user JSON object to the row array.
        }
        seatsJson.push_back(rowJson);  // Adds the row JSON array to the seats array.
    }
    
    json entityJson = {  // Creates a JSON object for the Train entity.
        {"trainId", entity.trainId},  // Stores the train's ID (note the key difference from Vehicle).
        {"name", entity.name},        // Stores the train's name.
        {"source", entity.source},    // Stores the train's source.
        {"destination", entity.destination},  // Stores the train's destination.
        {"time", entity.time},        // Stores the train's timestamp.
        {"seats", seatsJson}          // Stores the serialized seats array.
    };
    
    j.push_back(entityJson);  // Appends the Train JSON object to the main JSON array.
    
    ofstream outFile(filename);  // Opens the output file (db.json) for writing.
    outFile << j.dump(4);  // Writes the JSON data to the file with 4-space indentation for readability.
    outFile.close();  // Closes the output file.
}

template <>  // Explicit specialization of the FileIO class for Train type.
vector<Train> FileIO<Train>::readFromFile() {  // Defines the readFromFile method to deserialize Train objects from a JSON file.
    vector<Train> entities;  // Creates a vector to store the deserialized Train objects.
    ifstream file(filename);  // Opens the input file (db.json) for reading.
    
    if (!file.is_open()) {  // Checks if the file failed to open.
        return entities;  // Returns an empty vector if the file cannot be opened.
    }
    
    json j;  // Creates a JSON object to store the file's data.
    file >> j;  // Reads the JSON data from the file into the json object.
    file.close();  // Closes the input file.
    
    for (const auto& item : j) {  // Iterates over each JSON object in the JSON array.
        Train entity;  // Creates a new Train object to populate.
        entity.trainId = item["trainId"];  // Sets the trainId from the JSON object (note the key difference from Vehicle).
        entity.name = item["name"];  // Sets the name from the JSON object.
        entity.source = item["source"];  // Sets the source from the JSON object.
        entity.destination = item["destination"];  // Sets the destination from the JSON object.
        entity.time = item["time"];  // Sets the timestamp from the JSON object.
        
        // Convert seats back from JSON
        entity.seats.clear();  // Clears the seats vector to ensure it's empty before populating.
        for (const auto& rowJson : item["seats"]) {  // Iterates over each row in the seats JSON array.
            vector<User> row;  // Creates a vector for the current row of Users.
            for (const auto& userJson : rowJson) {  // Iterates over each user in the row JSON array.
                User user;  // Creates a new User object.
                user.userId = userJson["userId"];  // Sets the userId from the JSON object.
                user.name = userJson["name"];  // Sets the name from the JSON object.
                user.aadharCard = userJson["aadharCard"];  // Sets the Aadhar card number from the JSON object.
                row.push_back(user);  // Adds the User to the row vector.
            }
            entity.seats.push_back(row);  // Adds the row to the Train's seats vector.
        }
        
        entities.push_back(entity);  // Adds the populated Train to the entities vector.
    }
    
    return entities;  // Returns the vector of deserialized Train objects.
}

// Explicit template instantiations
template class FileIO<Vehicle>;  // Explicitly instantiates the FileIO class for Vehicle to ensure the template is compiled.
template class FileIO<Train>;    // Explicitly instantiates the FileIO class for Train to ensure the template is compiled.