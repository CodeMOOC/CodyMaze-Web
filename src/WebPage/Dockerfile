# ---
# First stage (build)
FROM mcr.microsoft.com/dotnet/sdk:6.0 AS build
WORKDIR /app

COPY *.csproj ./
RUN dotnet restore

COPY . ./
WORKDIR /app
RUN dotnet publish -c Release -o out

# ---
# Second stage (execution)
FROM mcr.microsoft.com/dotnet/aspnet:6.0 AS runtime

ENV ASPNETCORE_URLS http://+:8080
EXPOSE 8080

WORKDIR /app
COPY --from=build /app/out ./

ENTRYPOINT ["dotnet", "WebPage.dll"]
