FROM rust:1.77.2-bookworm AS api
WORKDIR /app
COPY ./api .
RUN cargo build --release

FROM node:22.2-bookworm AS ui
WORKDIR /app
COPY ./ui .
RUN npm install
RUN npm run build

FROM debian:bookworm
WORKDIR app
EXPOSE 80
EXPOSE 3000
RUN mkdir ./static
ENV DATABASE_URI=sqlite::memory:
ENV MIGRATE=no
COPY --from=ui /app/dist ./static
COPY --from=api /app/target/release/proyecto-final .
CMD ["./proyecto-final"]
