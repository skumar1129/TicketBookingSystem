#pragma once  // Prevents multiple inclusion of this header file during compilation, avoiding redefinition errors.

#include <iostream>  // Includes the standard input/output stream library for console operations (e.g., cout, cin).
#include <time.h>    // Includes the C time library to use time_t for handling time-related data.
#include <vector>    // Includes the vector library to use the std::vector class for dynamic arrays.
#include "user.hpp"  // Includes the user.hpp header file, which defines the User struct, allowing Train to reference User.

using namespace std;  // Brings the standard namespace into scope, allowing use of std:: types (e.g., string, vector) without explicit qualification. Note: This is generally avoided in header files to prevent namespace pollution.

struct Train {                    // Defines a struct named Train to represent a train entity.
    string trainId;              // A string member to store a unique identifier for the train.
    string name;                 // A string member to store the name of the train.
    string source;               // A string member to store the train's starting station (source).
    string destination;          // A string member to store the train's final station (destination).
    time_t time;                 // A time_t member to store the train's departure or arrival time as a timestamp.
    vector<vector<User>> seats;  // A 2D vector of User objects, representing the seating arrangement where each inner vector is a row or section of seats.
};  // Closing brace for the Train struct definition.