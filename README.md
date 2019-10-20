# kpi-web-messanger-api

### Makefile
Makefile contains some commands
1. ```make build-image``` - builds image that contains only
installed deps with tag chat-api-message
2. ```make run``` - runs application with mounting ```src``` dir into container.
Notice that app runs over ```watchexec```, that restarts app on every change in ```src```
dir.
3. ```make build``` - builds app and save binary to .build directory.
4. ```make clear``` - remove .build image

### How to start
To start app on *0.0.0.0:3000* use ```docker-compose up```,
which also starts postgres.