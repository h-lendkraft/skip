Docker commands to check Dockerfile
# Build the Docker image
- docker build -t speed-search-app .
# Run the container with required environment variables
- docker run -e PORT=3000 \
		-e SPEED_USER="BACKESHMOORHTY" \
		-e SPEED_PASSWD="backeshmoorhty123" \
		-e SPEED_BASE_URL="https://lookup.checkmycontent.site" \
		-e SPEED_SEARCH_APPEND="/Home/Search" \
		-e RUST_LOG="info,tower_http=info" \
		-p 3000:3000 \
		speed-search-app
# Stop all containers using the speed-search-app image
- docker stop $(docker ps -q --filter ancestor=speed-search-app)

For Docker compose changes
- The expose directive only makes the port available to linked services within Docker, while ports maps the container port to the host system.
TODO:
- [ ] If one of the multiple requests fails, we are ignoring for now. no empty item nothing
- [ ] Accesstoken based login. also route for generating access token and can only be accesses uding admin token. when booting the server should dump a admin token. this admin token can request tokens which can be used to access the appilcation
- [ ] Improve the parsers, email gets parsed as [email protected]
