FROM ubuntu:22.04

RUN apt-get update && apt-get install -y curl build-essential gcc libssl-dev pkg-config

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /workspace

COPY . /workspace
