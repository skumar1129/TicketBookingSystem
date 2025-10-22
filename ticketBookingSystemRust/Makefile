# Specifies the Rust compiler (cargo, equivalent to g++ in the C++ Makefile).
CARGO = cargo

# Specifies the target executable name, matching C++'s booking_system.
TARGET = booking_system

# Specifies the output directory for the compiled binary (cargo places it in target/debug or target/release).
# Matches C++'s TARGET output.
OUTPUT_DIR = target/debug

# Default target, equivalent to C++'s 'all' target.
all: $(TARGET)

# Build target: runs cargo build to compile the Rust project.
# Unlike C++, no explicit object files are needed; cargo handles compilation and linking.
$(TARGET):
    @echo "Building $(TARGET) with cargo..."  # Prints a message for clarity, similar to C++'s implicit linking output.
    $(CARGO) build  # Runs cargo build, producing the executable in target/debug/booking_system.
    @cp $(OUTPUT_DIR)/$(TARGET) .  # Copies the executable to the project root, mimicking C++'s $(TARGET) placement.

# Clean target: removes build artifacts, equivalent to C++'s clean target.
clean:
    @echo "Cleaning build artifacts..."  # Prints a message for clarity.
    $(CARGO) clean  # Runs cargo clean, removing the target directory (equivalent to rm -f $(OBJS) $(TARGET)).
    rm -f $(TARGET)  # Removes the copied executable from the project root, if it exists.

# Declares all and clean as phony targets, meaning they don't correspond to actual files.
# Matches C++'s .PHONY directive to prevent conflicts with files named 'all' or 'clean'.
.PHONY: all clean