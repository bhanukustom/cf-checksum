FROM --platform=linux/amd64 golang:1.25 AS build
WORKDIR /src
COPY go.mod main.go ./
RUN CGO_ENABLED=0 go build -o /checksum .

FROM --platform=linux/amd64 scratch
COPY --from=build /checksum /checksum
COPY data /data
EXPOSE 4000
ENTRYPOINT ["/checksum"]
