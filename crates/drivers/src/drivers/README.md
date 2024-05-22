# Driver

"Drivers" are traits implemented by the FFI host with methods we call from Rust.

Driver are created during initialization of the BIOS and are passed to appropriate "subsystems" during POST of the BIOS. When the BIOS is used to boot the (Sargon)OS, the OS creates "clients" and keeps them around for the duration of the OS lifetime.
