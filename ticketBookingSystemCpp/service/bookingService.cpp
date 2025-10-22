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
void BookingService<Vehicle>::cancelBooking(string entityId, string userId) {
    FileIO<Vehicle> file;
    // Assumes FileIO<Vehicle>::readFromFile() returns std::vector<Vehicle>
    // and FileIO<Vehicle>::saveToFile(const std::vector<Vehicle>&) overwrites the stored collection.
    auto vehicles = file.readFromFile();
    bool changed = false;

    for (auto &v : vehicles) {
        if (v.vehicleId == entityId) {
            // seats is a 2D vector: vector<vector<User>>
            for (auto rowIt = v.seats.begin(); rowIt != v.seats.end();) {
                auto &row = *rowIt;
                for (auto userIt = row.begin(); userIt != row.end();) {
                    if (userIt->userId == userId) {
                        userIt = row.erase(userIt);
                        changed = true;
                    } else {
                        ++userIt;
                    }
                }
                // remove empty rows
                if (row.empty()) {
                    rowIt = v.seats.erase(rowIt);
                } else {
                    ++rowIt;
                }
            }
            break;
        }
    }

    if (changed) {
        for (const auto &veh : vehicles) {
            file.saveToFile(veh);
        }
        std::cout << "Cancelled booking for user " << userId << " on vehicle " << entityId << '\n';
    } else {
        std::cout << "No matching booking found for user " << userId << " on vehicle " << entityId << '\n';
    }
}

template <>  // Explicit specialization of the BookingService class for Vehicle type.
void BookingService<Vehicle>::printBooking(string entityId, string userId) {
    FileIO<Vehicle> file;
    // Assumes FileIO<Vehicle>::readFromFile() returns std::vector<Vehicle>
    auto vehicles = file.readFromFile();

    for (const auto &v : vehicles) {
        if (v.vehicleId == entityId) {
            std::cout << "Vehicle ID: " << v.vehicleId << "\n";
            std::cout << "Name: " << v.name << "\n";
            std::cout << "Source: " << v.source << "  Destination: " << v.destination << "\n";
            std::cout << "Time: " << v.time << "\n";
            bool found = false;
            for (size_t r = 0; r < v.seats.size(); ++r) {
                for (size_t c = 0; c < v.seats[r].size(); ++c) {
                    const auto &u = v.seats[r][c];
                    if (u.userId == userId) {
                        std::cout << "Booked seat - row: " << r << " col: " << c
                                  << " | User ID: " << u.userId
                                  << " | Name: " << u.name << '\n';
                        found = true;
                    }
                }
            }
            if (!found) {
                std::cout << "No booking found for user " << userId << " on vehicle " << entityId << '\n';
            }
            return;
        }
    }

    std::cout << "Vehicle with ID " << entityId << " not found\n";
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
void BookingService<Train>::cancelBooking(string entityId, string userId) {
    FileIO<Train> file;
    auto trains = file.readFromFile();
    bool changed = false;

    for (auto &t : trains) {
        if (t.trainId == entityId) {
            for (auto rowIt = t.seats.begin(); rowIt != t.seats.end();) {
                auto &row = *rowIt;
                for (auto userIt = row.begin(); userIt != row.end();) {
                    if (userIt->userId == userId) {
                        userIt = row.erase(userIt);
                        changed = true;
                    } else {
                        ++userIt;
                    }
                }
                if (row.empty()) {
                    rowIt = t.seats.erase(rowIt);
                } else {
                    ++rowIt;
                }
            }
            break;
        }
    if (changed) {
        for (const auto &tr : trains) {
            file.saveToFile(tr);
        }
        std::cout << "Cancelled booking for user " << userId << " on train " << entityId << '\n';
    } else {
        std::cout << "No matching booking found for user " << userId << " on train " << entityId << '\n';
    }
        std::cout << "No matching booking found for user " << userId << " on train " << entityId << '\n';
    }
}

template <>  // Explicit specialization of the BookingService class for Train type.
void BookingService<Train>::printBooking(string entityId, string userId) {
    FileIO<Train> file;
    auto trains = file.readFromFile();

    for (const auto &t : trains) {
        if (t.trainId == entityId) {
            std::cout << "Train ID: " << t.trainId << "\n";
            std::cout << "Name: " << t.name << "\n";
            std::cout << "Source: " << t.source << "  Destination: " << t.destination << "\n";
            std::cout << "Time: " << t.time << "\n";
            bool found = false;
            for (size_t r = 0; r < t.seats.size(); ++r) {
                for (size_t c = 0; c < t.seats[r].size(); ++c) {
                    const auto &u = t.seats[r][c];
                    if (u.userId == userId) {
                        std::cout << "Booked seat - row: " << r << " col: " << c
                                  << " | User ID: " << u.userId
                                  << " | Name: " << u.name << '\n';
                        found = true;
                    }
                }
            }
            if (!found) {
                std::cout << "No booking found for user " << userId << " on train " << entityId << '\n';
            }
            return;
        }
    }

    std::cout << "Train with ID " << entityId << " not found\n";
}

template <>  // Explicit specialization of the BookingService class for Train type.
void BookingService<Train>::saveBooking(Train entity) {  // Defines the saveBooking method for Train.
    FileIO<Train> file;  // Creates a FileIO object for Train to handle file operations.
    file.saveToFile(entity);  // Calls the saveToFile method to serialize and save the Train object to a file (e.g., db.json).
}