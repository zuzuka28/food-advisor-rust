FROM  rust:latest AS build

WORKDIR /food-adviser-api

COPY . .

RUN cargo build --release

FROM  rust:latest

COPY --from=build /food-adviser-api/target/release/food-adviser-api .

CMD ["./food-adviser-api"]
