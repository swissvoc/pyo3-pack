FROM quay.io/pypa/manylinux1_x86_64

ENV PATH /root/.cargo/bin:$PATH
# Add all supported python versions
ENV PATH /opt/python/cp27-cp27mu/bin/:/opt/python/cp35-cp35m/bin/:/opt/python/cp36-cp36m/bin/:/opt/python/cp37-cp37m/bin/:$PATH
# Otherwise `cargo new` errors
ENV USER root

RUN curl https://www.musl-libc.org/releases/musl-1.1.20.tar.gz -o musl.tar.gz \
    && tar -xzf musl.tar.gz \
    && rm -f musl.tar.gz \
    && cd musl-1.1.20 \
    && ./configure \
    && make install -j2 \
    && cd .. \
    && rm -rf x86_64-unknown-linux-musl \
    && curl https://sh.rustup.rs -sSf | sh -s -- -y \
    && rustup toolchain add nightly \
    && rustup target add x86_64-unknown-linux-musl \
    && mkdir /io \
    && python3 -m pip install cffi

ADD . /pyo3-pack/

# We can't use the upload feature because either the openssl is too old (for native) or the perl version is too old (for vendored)
RUN cargo install --path /pyo3-pack --no-default-features --features "auditwheel log"

WORKDIR /io

ENTRYPOINT ["/root/.cargo/bin/pyo3-pack"]
