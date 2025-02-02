# XAPK Extractor

Rust application that installs XAPK files using the `adb` command.

## Usage

Takes XAPK file in given input, will extract the XAPK into it's own directory, and install them using the `adb` command. Ensure that your device is connected and `adb` is set up correctly with `adb devices` check.

## Requirements
- Rust (latest)
- ADB (Android Debug Bridge) installed and configured

## Building the Project

To build the project, navigate to the project directory and run:

```
cargo build --release
```

## Running the Application

To run the application, use the following command:

```
cargo run [path to xapk]
```

Further tasks:
- [ ] Incorporate [`apkeep`](https://github.com/EFForg/apkeep).
- [ ] Install `adb` for user if it's not.
- [ ] Package as a global tool
