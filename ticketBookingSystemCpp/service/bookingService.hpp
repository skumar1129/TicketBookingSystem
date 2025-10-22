#pragma once  // Prevents multiple inclusion of this header file during compilation, avoiding redefinition errors.

#include <iostream>  // Includes the standard input/output stream library for console operations (e.g., cout, cin).
#include <ctime>     // Includes the C time library to use time-related types or functions (e.g., time_t), though not directly used in this code snippet.
#include "../entities/user.hpp"    // Includes the user.hpp header file, which defines the User struct, used in method parameters.
#include "../entities/vehicle.hpp" // Includes the vehicle.hpp header file, which defines the Vehicle struct, used as a template parameter.
#include "../entities/train.hpp"   // Includes the train.hpp header file, which defines the Train struct, used as a template parameter.
#include "fileIO.hpp"              // Includes the fileIO.hpp header file, which defines the FileIO class for file operations.

using namespace std;  // Brings the standard namespace into scope, allowing use of std:: types (e.g., string, vector) without explicit qualification. Note: This is generally avoided in header files to prevent namespace pollution.

template <typename T>  // Defines a generic template class BookingService for handling booking operations with type T.
class BookingService {  // Declaration of the BookingService class.
    public:
        // Declares a method to book a seat for a user on an entity (Vehicle or Train) identified by entityId, using user details, name, source, and destination.
        void book(string entityId, User user, string name, string source, string destination);
        
        // Declares a method to cancel a booking for a user identified by userId on an entity identified by entityId.
        void cancelBooking(string entityId, string userId);
        
        // Declares a method to print booking details for a user identified by userId on an entity identified by entityId.
        void printBooking(string entityId, string userId);

    private:
        // Defines a private method to save a booking by writing the entity (of type T) to a file using FileIO.
        void saveBooking(T entity) {
            FileIO<T> file;         // Creates a FileIO object for the specified type T to handle file operations.
            file.saveToFile(entity); // Calls the saveToFile method to persist the entity to a file (e.g., db.json).
        }
};

// Template specializations for Vehicle
template <>  // Explicit specialization for Vehicle type.
void BookingService<Vehicle>::saveBooking(Vehicle entity);  // Declares a specialized saveBooking method for Vehicle objects.

// Template specializations for Train
template <>  // Explicit specialization for Train type.
void BookingService<Train>::saveBooking(Train entity);      // Declares a specialized saveBooking method for Train objects.