# My Rust Project

This project is a Rust application that installs XAPK files using the ADB command.

## Requirements
- Rust (latest)
- ADB (Android Debug Bridge) installed and configured
[TODO]: Might add support to install for the user!

## Building the Project

To build the project, navigate to the project directory and run:

```
cargo build --release
```

## Running the Application

To run the application, use the following command:

```
cargo run
```
Make sure to provide the necessary APK files in the specified directory.

## Usage

XAPK files in given directory, will extract the XAPK into it's own directory, and install them using the `adb` command. Ensure that your device is connected and `adb` is set up correctly.