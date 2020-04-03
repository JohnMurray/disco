# Disco Router

## Capabilities

Listening to a raw socket on Linux is a privileged operation, you can either
run the binary with `sudo`, or you can add the necessary capability:

```bash
# Example: Build and set capability on resulting binary
> cargo build
> sudo setcap cap_net_raw=ep target/debug/disco
> ./target/debug/disco
# => capability set for specific binary to listen to raw sockets. Process
#    spawned from this file will inherit this capability.

> getcap target/debug/disco
# => Verify it is set properly
# => target/debug/disco = cap_net_raw+ep
```
