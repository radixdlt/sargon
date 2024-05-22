# Subsystem

A subsystem needs a "driver" (just like a "client"), but they don't need to be owned by the SargonOS and is never directly accessed, since they have have a static instance with their own lifecycle.

An example of a subsystem is the `log` for which we install a Driver from FFI host, and is initialized
during creation of the BIOS, but it is not a client, since we wanna be able to log from anywhere inside of the Sargon crate, not specifically tied to the SargonOS instance.
