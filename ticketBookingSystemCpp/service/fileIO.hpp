#pragma once  // Prevents multiple inclusion of this header file during compilation, avoiding redefinition errors.

#include <iostream>  // Includes the standard input/output stream library for console operations (e.g., cout, cin).
#include <fstream>   // Includes the file stream library for file input/output operations (e.g., ofstream, ifstream).
#include <vector>    // Includes the vector library to use the std::vector class for dynamic arrays.
#include "../external/json.hpp"  // Includes a JSON library (likely nlohmann/json) for handling JSON serialization/deserialization.
#include "../entities/vehicle.hpp"  // Includes the vehicle.hpp header file, which defines the Vehicle struct.
#include "../entities/train.hpp"    // Includes the train.hpp header file, which defines the Train struct.

using namespace std;  // Brings the standard namespace into scope, allowing use of std:: types (e.g., string, vector) without explicit qualification.
using json = nlohmann::json;  // Creates an alias for the nlohmann::json type, simplifying JSON-related operations.

template <typename T>  // Defines a generic template class FileIO for handling file operations with type T.
class FileIO {         // Declaration of the FileIO class.
    private:
        const string filename = "db.json";  // A constant string member specifying the default file name ("db.json") for data storage.

    public:
        void saveToFile(T entity);      // Declares a public method to save a single entity of type T to the file.
        vector<T> readFromFile();       // Declares a public method to read a vector of entities of type T from the file.
};

// Forward declarations of template specializations
template <>  // Explicit specialization for Vehicle type.
void FileIO<Vehicle>::saveToFile(Vehicle entity);  // Declares a specialized method to save a Vehicle object to the file.
template <>  // Explicit specialization for Vehicle type.
vector<Vehicle> FileIO<Vehicle>::readFromFile();  // Declares a specialized method to read a vector of Vehicle objects from the file.
template <>  // Explicit specialization for Train type.
void FileIO<Train>::saveToFile(Train entity);     // Declares a specialized method to save a Train object to the file.
template <>  // Explicit specialization for Train type.
vector<Train> FileIO<Train>::readFromFile();      // Declares a specialized method to read a vector of Train objects from the file.