FROM microsoft/dotnet:2.1-sdk AS build
WORKDIR /app
COPY . ./
RUN dotnet restore && dotnet publish -c Release -r linux-musl-x64

FROM microsoft/dotnet:2.1-runtime-deps-alpine
COPY --from=build /app/bin/Release/*/linux-musl-x64/publish /app
RUN chmod +x /app/ddns-update
ENTRYPOINT ["/app/ddns-update"]
