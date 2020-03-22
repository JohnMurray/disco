#!/bin/bash -l

BUILD_DIR=/build

##-----------------------------------------------------------------------------
## Utility Functions
##

silent_pushd() {
    pushd $1 > /dev/null 2>&1
}


##-----------------------------------------------------------------------------
## Arg Parsing
##

while (( "$#" )); do
  case "$1" in
    # FIXME: Removed, but leaving for docs on how to do nice flag reading in bash
    #        ::shrug::
    ## -n|--no-patch)
    ##   NO_PATCH=1
    ##   shift
    ##   ;;
    *) # unsupported flags or trailing arguments
      echo "Error: Unsupported flag $1" >&2
      exit 1
      ;;
  esac
done

##-----------------------------------------------------------------------------
## Environment Setup and Checks
##

# Check that the build dir exists and is executable (can CD into it)
if [ ! -e $BUILD_DIR ] || [ ! -x $BUILD_DIR ] ; then
    echo "There is no build-directory at /build, please ensure you are running"
    echo "the docker container with the correct mount, such as:"
    echo "  docker run --mount type=bind,src=($pwd),dst=/build"
    exit 1
fi
silent_pushd $BUILD_DIR

# Check that the directory contains a Cargo.toml and is readable
if [ ! -e 'Cargo.toml' ] ; then
    echo "No 'Cargo.toml' found within the build directory. Cannot build"
    exit 1
fi
if [ ! -r 'Cargo.toml' ] ; then
    echo "'Cargo.toml' file is not readable. Cannot build"
    exit 1
fi

cargo build \
    --target=armv7-unknown-linux-gnueabihf; ret=$?

if [ "$ret" -ne "0" ] ; then
    echo "Non-zero exit code encountered while running 'cargo build' ($ret)"
    exit 1
fi