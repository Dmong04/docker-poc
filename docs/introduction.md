# Introducing Docker

Here I learned about Docker, create docker images and deploy an API build in Rust, with docker reducing the disk usage,
the content size and the time to deploy the application.

I reach that state with the following steps:

```
    # Command to build the docker image
    docker build -t docker-practica:rust-1.0 .

    # Command to compare the image sizes
    docker images

    # Command to run the docker image
    docker run -d -p 8080:5000 docker-practica:rust-1.0

    # Command to check the running containers with curl (you can check the container with Postman or your browser too)
    curl http://localhost:8080
    curl http://localhost:8080/health

    # Check the logs of the running container
    docker logs <YOUR_CONTAINER_ID>
```

## Disk Usage and Content Size

**Comparing Disk usage and content size between local and Docker images**

| Image                    | Disk Usage | content Size |
| ------------------------ | ---------- | ------------ |
| docker-practica:rust-1.0 | 116MB      | 29MB         |
| rust:1-slim              | 1.28GB     | 334MB        |
