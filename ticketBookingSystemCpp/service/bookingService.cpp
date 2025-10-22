#include "bookingService.hpp"  // Includes the bookingService.hpp header file, which declares the BookingService template class and its methods.
#include <ctime>              // Includes the C time library to use time-related functions (e.g., time(nullptr)) for setting timestamps.

// Template implementation for Vehicle
template <>  // Explicit specialization of the BookingService class for Vehicle type.
void BookingService<Vehicle>::book(string entityId, User user, string name, string source, string destination) {  // Defines the book method to create a booking for a Vehicle.
    Vehicle vehicle;  // Creates a new Vehicle object to represent the booked entity.
    vehicle.vehicleId = entityId;  // Sets the vehicle's ID from the provided entityId.
    vehicle.name = name;          // Sets the vehicle's name from the provided name.
    vehicle.source = source;      // Sets the vehicle's source from the provided source.
    vehicle.destination = destination;  // Sets the vehicle's destination from the provided destination.
    vehicle.time = time(nullptr);  // Sets the vehicle's timestamp to the current time using time(nullptr).
    vehicle.seats = {{user}};     // Initializes the seats as a 2D vector with a single row containing the provided User object.
    
    saveBooking(vehicle);  // Calls the saveBooking method to persist the Vehicle object to a file.
}

template <>  // Explicit specialization of the BookingService class for Vehicle type.
void BookingService<Vehicle>::cancelBooking(string entityId, string userId) {  // Defines the cancelBooking method for Vehicle (placeholder).
    // Implementation for cancel booking
    // No implementation provided; likely intended to remove a user's booking from a Vehicle identified by entityId.
}

template <>  // Explicit specialization of the BookingService class for Vehicle type.
void BookingService<Vehicle>::printBooking(string entityId, string userId) {  // Defines the printBooking method for Vehicle (placeholder).
    // Implementation for print booking
    // No implementation provided; likely intended to display booking details for a user on a Vehicle.
}

template <>  // Explicit specialization of the BookingService class for Vehicle type.
void BookingService<Vehicle>::saveBooking(Vehicle entity) {  // Defines the saveBooking method for Vehicle.
    FileIO<Vehicle> file;  // Creates a FileIO object for Vehicle to handle file operations.
    file.saveToFile(entity);  // Calls the saveToFile method to serialize and save the Vehicle object to a file (e.g., db.json).
}

// Template implementation for Train
template <>  // Explicit specialization of the BookingService class for Train type.
void BookingService<Train>::book(string entityId, User user, string name, string source, string destination) {  // Defines the book method to create a booking for a Train.
    Train train;  // Creates a new Train object to represent the booked entity.
    train.trainId = entityId;  // Sets the train's ID from the provided entityId.
    train.name = name;        // Sets the train's name from the provided name.
    train.source = source;    // Sets the train's source from the provided source.
    train.destination = destination;  // Sets the train's destination from the provided destination.
    train.time = time(nullptr);  // Sets the train's timestamp to the current time using time(nullptr).
    train.seats = {{user}};     // Initializes the seats as a 2D vector with a single row containing the provided User object.
    
    saveBooking(train);  // Calls the saveBooking method to persist the Train object to a file.
}

template <>  // Explicit specialization of the BookingService class for Train type.
void BookingService<Train>::cancelBooking(string entityId, string userId) {  // Defines the cancelBooking method for Train (placeholder).
    // Implementation for cancel booking
    // No implementation provided; likely intended to remove a user's booking from a Train identified by entityId.
}

template <>  // Explicit specialization of the BookingService class for Train type.
void BookingService<Train>::printBooking(string entityId, string userId) {  // Defines the printBooking method for Train (placeholder).
    // Implementation for print booking
    // No implementation provided; likely intended to display booking details for a user on a Train.
}

template <>  // Explicit specialization of the BookingService class for Train type.
void BookingService<Train>::saveBooking(Train entity) {  // Defines the saveBooking method for Train.
    FileIO<Train> file;  // Creates a FileIO object for Train to handle file operations.
    file.saveToFile(entity);  // Calls the saveToFile method to serialize and save the Train object to a file (e.g., db.json).
}