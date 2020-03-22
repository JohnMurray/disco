# PI-Builder

Because getting the right cross-compiling libs in arch has caused me more headaches
than not, I've put together a docker-container to build the application. The
docker-file can be built and used as such:

```sh
# Build & Name Image
docker build -t pi_builder:latest .

# Run image on local dir
docker run --mount type=bind,src=$(pwd),dst=/build pi_builder:latest
```

But this isn't fun to run every time and it's hard to remember. Instead there is a script,
`rpb` (raspberry-pi builder), that can be used much easier:

```sh
> rpb
# => If no docker image found for pi_builder:latest, it will build one.
# => runs build in the current directory

> rpb build
# => Same thing as `rpb`, just more explicit

> rpb update
> rpb up
# => Updates the docker image with a no-cache rebuild
```

Just add the `rpb` script to your `PATH` and you're good to go. There is a `.profile`
in this folder you can source to add it to your current shell's `PATH`.