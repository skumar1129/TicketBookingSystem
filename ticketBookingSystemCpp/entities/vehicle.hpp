#pragma once  // Prevents multiple inclusion of this header file during compilation, avoiding redefinition errors.

#include <iostream>  // Includes the standard input/output stream library for console operations (e.g., cout, cin).
#include <time.h>    // Includes the C time library to use time_t for handling time-related data.
#include <vector>    // Includes the vector library to use the std::vector class for dynamic arrays.
#include "user.hpp"  // Includes the user.hpp header file, which defines the User struct, used in the seats member.
#include "train.hpp" // Includes the train.hpp header file, which defines the Train struct, used in the convert function.

using namespace std;  // Brings the standard namespace into scope, allowing use of std:: types (e.g., string, vector) without explicit qualification. 

struct Vehicle {                  // Defines a struct named Vehicle to represent a generic vehicle entity.
    string vehicleId;             // A string member to store a unique identifier for the vehicle.
    string name;                  // A string member to store the name of the vehicle.
    string source;                // A string member to store the vehicle's starting point (source).
    string destination;           // A string member to store the vehicle's end point (destination).
    time_t time;                  // A time_t member to store the vehicle's departure or arrival time as a timestamp.
    vector<vector<User>> seats;   // A 2D vector of User objects, representing the seating arrangement where each inner vector is a row or section of seats.
};  // Closing brace for the Vehicle struct definition.

inline Train convert(Vehicle vehicle) {  // Defines an inline function to convert a Vehicle object to a Train object.
    Train train;                        // Creates a new Train object to store the converted data.
    train.trainId = vehicle.vehicleId;  // Copies the vehicleId from Vehicle to trainId in Train.
    train.name = vehicle.name;          // Copies the name from Vehicle to name in Train.
    train.source = vehicle.source;      // Copies the source from Vehicle to source in Train.
    train.destination = vehicle.destination;  // Copies the destination from Vehicle to destination in Train.
    train.time = vehicle.time;          // Copies the time from Vehicle to time in Train.
    train.seats.resize(vehicle.seats.size());  // Resizes the train's seats vector to match the size of the vehicle's seats vector.
    for(size_t i = 0; i < vehicle.seats.size(); i++) {  // Iterates over each row/section in the vehicle's seats.
        train.seats[i] = vehicle.seats[i];  // Copies each row/section of User objects from Vehicle to Train.
    }
    return train;                       // Returns the populated Train object.
}  // Closing brace for the convert function.