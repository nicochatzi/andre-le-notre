FROM --platform=linux/amd64 nicochatzi/lenotre as base


# RUN cargo init
# COPY Cargo.toml /code/Cargo.toml
# COPY Cargo.lock /code/Cargo.lock
# RUN mkdir -p .cargo \
#   && cargo vendor > .cargo/config

RUN mkdir /app
WORKDIR /app
COPY . /app

FROM base AS test
RUN npm i
# CMD [ "cargo", "test"",, "--offline"]
CMD [ "cargo", "test" ]

FROM base AS run
RUN npm i
CMD [ "npm", "start" ]
