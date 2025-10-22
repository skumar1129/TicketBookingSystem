#pragma once  // Prevents multiple inclusion of this header file during compilation, ensuring no redefinition errors.

#include <iostream>  // Includes the standard input/output stream library for console operations (e.g., cout, cin).
#include <string>    // Includes the string library to use the std::string class for handling text.
#include <vector>    // Includes the vector library to use the std::vector class for dynamic arrays.

using namespace std;  // Brings the standard namespace into scope, allowing use of std:: types (e.g., string, vector) without explicit qualification. Note: This is generally avoided in header files to prevent namespace pollution.

struct Vehicle;  // Forward declaration of the Vehicle struct, allowing the User struct to reference Vehicle pointers without needing the full definition.

struct User {              // Defines a struct named User to represent a user entity.
    string userId;         // A string member to store a unique identifier for the user.
    string name;           // A string member to store the user's name.
    string aadharCard;     // A string member to store the user's Aadhar card number (a unique ID in India).
    vector<Vehicle*> vehicles;  // A vector of pointers to Vehicle objects, representing vehicles associated with the user.
};  // Closing brace for the User struct definition.