# SeaHorn builder image that builds binary SeaHorn release package
# Primarily used by the CI
# Arguments:
#  - BASE-IMAGE: jammy-llvm14
#  - BUILD_TYPE: Debug, RelWithDebInfo, Coverage
ARG BASE_IMAGE=jammy-llvm14
FROM seahorn/buildpack-deps-seahorn:$BASE_IMAGE

# Assume that docker-build is ran in the top-level SeaHorn directory
COPY submodules/seahorn /seahorn

# Re-create the build directory that might have been present in the source tree
RUN rm -rf /seahorn/build /seahorn/debug /seahorn/release && \
  mkdir /seahorn/build && \
# Remove any third-party dependencies that build process clones
  rm -rf /seahorn/clam /seahorn/sea-dsa /seahorn/llvm-seahorn
WORKDIR /seahorn/build

ARG BUILD_TYPE=RelWithDebInfo

# Build configuration
RUN --mount=type=cache,target=/opt/cache/seahorn_make cmake .. -GNinja \
  -DCMAKE_BUILD_TYPE=${BUILD_TYPE} \
  -DZ3_ROOT=/opt/z3-4.8.9 \
  -DYICES2_HOME=/opt/yices-2.6.1 \
  -DCMAKE_INSTALL_PREFIX=run \
  -DCMAKE_CXX_COMPILER=clang++-14 \
  -DCMAKE_C_COMPILER=clang-14 \
  -DSEA_ENABLE_LLD=ON \
  -DCPACK_GENERATOR="TGZ" \
  -DCMAKE_EXPORT_COMPILE_COMMANDS=ON && \
  cmake --build . --target extra  && cmake .. && \
  cmake --build . --target crab  && cmake .. && \
  cmake --build . --target install && \
  cmake --build . --target units_z3 && \
  cmake --build . --target units_yices2 && \
  cmake --build . --target test_type_checker && \
  cmake --build . --target test_hex_dump && \
  cmake --build . --target package && \
  units/units_z3 && \
  units/units_yices2

ENV PATH "/seahorn/build/run/bin:$PATH"
WORKDIR /seahorn


############################################
# Stage 2 — Final Image (SeaHorn + AutoVerus)
############################################
# FROM ubuntu:22.04

ENV DEBIAN_FRONTEND=noninteractive
ENV PYTHONDONTWRITEBYTECODE=1
ENV PYTHONUNBUFFERED=1
ENV RUST_BACKTRACE=1

# install add-apt-repository command
RUN apt-get update && apt-get install -y software-properties-common
# add ppa for yices
RUN add-apt-repository ppa:sri-csl/formal-methods


############################################
# Install system dependencies
############################################
RUN apt-get update && apt-get install -y \
    python3.10 python3.10-dev python3-pip python3.10-venv \
    curl build-essential pkg-config libssl-dev \
    git vim unzip sudo wget \
    ca-certificates gnupg lsb-release \
    yices2 \
    z3 \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

############################################
# Install Yices 2.6.1 (same as stage 1)
############################################
# RUN wget --user-agent="Mozilla" \
#     https://yices.csl.sri.com/releases/2.6.1/yices-2.6.1-x86_64-pc-linux-gnu.tar.gz \
#     -O yices.tar.gz && \
#     tar -xzf yices.tar.gz && \
#     rm yices.tar.gz && \
#     cd yices-2.6.1 && \
#     ./install-yices --prefix=/usr/local && \
#     cd .. && rm -rf yices-2.6.1

# Seahorn depends on this exact name:
RUN ln -s /usr/local/lib/libyices.so /usr/local/lib/libyices.so.2.6 || true

############################################
# Create non-root user
############################################
RUN useradd -m -s /bin/bash appuser && \
    adduser appuser sudo && \
    echo "appuser ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers

USER appuser
WORKDIR /home/appuser

############################################
# Copy SeaHorn from stage 1
############################################
# COPY --from=seahorn-builder /seahorn/ /seahorn/
ENV PATH="/seahorn/build/run/bin:${PATH}"

############################################
# Install Rust (nightly + stable 1.66)
############################################
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
    | sh -s -- --default-toolchain none -y

ENV PATH="/home/appuser/.cargo/bin:${PATH}"

# Install required toolchains
RUN --mount=type=cache,target=/opt/cache/rustup rustup toolchain install nightly && \
    rustup toolchain install 1.66.0 && \
    rustup default nightly-2022-03-01

############################################
# Install crux-mir
############################################
# mir-json
# proper rust version
RUN rustup toolchain install nightly-2025-09-14 --force --component rustc-dev,rust-src
COPY mir-json-ubuntu-22.04-X64/ /mir-json
ENV PATH="/mir-json/bin:${PATH}"
ENV CRUX_RUST_LIBRARY_PATH="/mir-json/rlibs"

# install haskell
ENV BOOTSTRAP_HASKELL_NONINTERACTIVE=1
RUN curl --proto '=https' --tlsv1.2 -sSf https://get-ghcup.haskell.org | sh
ENV PATH="/home/appuser/.ghcup/bin:${PATH}"

# RUN sudo apt-get update && sudo apt-get install -y golang

# clone crucible
WORKDIR /home/appuser
RUN git clone https://github.com/GaloisInc/crucible.git
WORKDIR /home/appuser/crucible/crux-mir
RUN git submodule update --init --recursive
RUN cabal update
RUN cabal install exe:crux-mir

# install rust toolchain for lynnette
RUN rustup toolchain install 1.91.0-x86_64-unknown-linux-gnu


############################################
# Copy Verus-Proof-Synthesis submodule
############################################
COPY --chown=appuser:appuser submodules/verus-proof-synthesis /home/appuser/verus-proof-synthesis

COPY --chown=appuser:appuser test_cases /home/appuser/verus-proof-synthesis/test_cases
COPY --chown=appuser:appuser seahorn_script.sh /home/appuser/verus-proof-synthesis/seahorn_script.sh

############################################
# Build Verus
############################################
WORKDIR /home/appuser/verus-proof-synthesis
RUN mkdir smt_output
RUN chmod +x seahorn_script.sh
RUN git clone https://github.com/verus-lang/verus.git

WORKDIR /home/appuser/verus-proof-synthesis/verus
RUN git checkout 33269ac6a0ea33a08109eefe5016c1fdd0ce9fbd

WORKDIR /home/appuser/verus-proof-synthesis/verus/source
RUN --mount=type=cache,target=/opt/cache/verus_build bash -c "source ../tools/activate && ./tools/get-z3.sh && vargo build --release"

############################################
# Create Python venv for AutoVerus
############################################
WORKDIR /home/appuser/verus-proof-synthesis
RUN python3.10 -m venv /home/appuser/venv

ENV PATH="/home/appuser/venv/bin:/home/appuser/verus-proof-synthesis/verus/source/target-verus/release:${PATH}"

RUN pip install --upgrade pip setuptools wheel
RUN pip install -r /home/appuser/verus-proof-synthesis/requirements.txt

# just put here cause don't want to rebuild
ENV PATH="/home/appuser/.cabal/bin:${PATH}"

############################################
# Optional Azure CLI
############################################
RUN curl -sL https://aka.ms/InstallAzureCLIDeb | sudo bash

CMD ["/bin/bash"]

LABEL maintainer="you"
LABEL description="SeaHorn + AutoVerus combined container"
LABEL version="1.0"
