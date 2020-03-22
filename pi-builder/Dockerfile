FROM ubuntu:19.10

##-----------------------------------------------------------------------------
## Install some basic tools/libs
RUN apt-get update && apt-get install -y \
    git                                  \
    build-essential                      \
    curl                                 \
    libssl-dev                           \
    zlib1g-dev

RUN apt-get install -y gcc-9-multilib-arm-linux-gnueabihf

##-----------------------------------------------------------------------------
## Setup execution environment
RUN useradd -ms /bin/bash rust
USER rust
ENV HOME /home/rust
ENV USER rust
ENV SHELL /bin/bash
WORKDIR /home/rust


##-----------------------------------------------------------------------------
## Install Rust Toolchain
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | \
    sh -s -- -y
RUN echo 'export PATH=~/.cargo/bin:$PATH' > ~/.bashrc

RUN ~/.cargo/bin/rustup target add armv7-unknown-linux-gnueabihf

RUN echo ""                                        >> ~/.cargo/config && \
    echo "[target.armv7-unknown-linux-gnueabihf]"  >> ~/.cargo/config && \
    echo "linker = \"arm-linux-gnueabihf-gcc-9\""  >> ~/.cargo/config

##-----------------------------------------------------------------------------
## 


##-----------------------------------------------------------------------------
## docker run -ti --mount type=bind,src=/root/build,dst=$(pwd) IMAGE_NAME:latest ...
COPY pi-build.sh pi-build
ENTRYPOINT ["./pi-build"]