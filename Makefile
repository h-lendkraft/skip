.PHONY: build run login search-mobile health clean stop logs all

# Configuration variables - edit these as needed
PORT ?= 3000
SPEED_USER ?= boneykumar@123
SPEED_PASSWD ?= @boney123!\&
SPEED_BASE_URL ?= https://search.findcustomersdata.online
API_BASE_URL ?= http://localhost:$(PORT)

#
# Test commands - step by step testing
health:
	@echo "Testing health endpoint..."
	curl -s $(API_BASE_URL)/health | jq

login:
	@echo "Testing login functionality..."
	curl -s -X GET $(API_BASE_URL)/speed/login 

search-mobile:
	@if [ -z "$(NUMBERS)" ]; then \
		echo "Error: Please provide mobile numbers. Usage: make search NUMBERS=\"1111111111\" or make search NUMBERS=\"111111111 2222222222\""; \
		exit 1; \
	fi; \
	for number in $(NUMBERS); do \
		echo "Searching for $$number..."; \
		curl -s -X POST $(API_BASE_URL)/speed/search/mobile \
			-H "Content-Type: application/json" \
			-d "[\"$$number\"]" | jq; \
		echo "\n"; \
	done

search-aadhar:
	@if [ -z "$(AADHARS)" ]; then \
		echo "Error: Please provide aadhar. Usage: make search AADHARS=\"111111111111\" or make search AADHARS=\"11111111111 222222222222\""; \
		exit 1; \
	fi; \
	for number in $(AADHARS); do \
		echo "Searching for $$number..."; \
		curl -s -X POST $(API_BASE_URL)/speed/search/aadhar \
			-H "Content-Type: application/json" \
			-d "[\"$$number\"]" | jq; \
		echo "\n"; \
	done
# Build and run commands
build:
	@echo "Building the application..."
	cargo build

run:
	@echo "Starting the application on port $(PORT)..."
	RUST_LOG=info PORT=$(PORT) SPEED_USER=$(SPEED_USER) SPEED_PASSWD=$(SPEED_PASSWD) SPEED_BASE_URL=$(SPEED_BASE_URL) cargo run

run-docker:
	@echo "Building and running the Docker container..."
	docker-compose up --build -d

# Validation test
test-validation:
	@echo "Testing validation (invalid mobile number)..."
	curl -s -X POST $(API_BASE_URL)/speed/search \
		-H "Content-Type: application/x-www-form-urlencoded" \
		-d "mobile=12345" | jq

# Sequential testing workflow
test-flow: health login search-user

# Full testing workflow
test-all: health login search-user search-multiple test-validation

# Utility commands
clean:
	@echo "Cleaning up..."
	cargo clean

stop:
	@echo "Stopping Docker containers..."
	docker-compose down

logs:
	@echo "Showing logs..."
	docker-compose logs -f

# Help command
help:
	@echo "Available commands:"
	@echo "  make build            - Build the application"
	@echo "  make run              - Run the application locally"
	@echo "  make run-docker       - Run the application in Docker"
	@echo "  make health           - Test the health endpoint"
	@echo "  make login            - Test login functionality"
	@echo "  make search-user      - Test searching for a user"
	@echo "  make search-multiple  - Test multiple search requests"
	@echo "  make test-validation  - Test input validation"
	@echo "  make test-flow        - Run a basic end-to-end test flow"
	@echo "  make test-all         - Run all tests"
	@echo "  make clean            - Clean the build"
	@echo "  make stop             - Stop Docker containers"
	@echo "  make logs             - Show Docker logs"
	@echo ""
	@echo "You can override the configuration by setting environment variables:"
	@echo "  make search-user MOBILE_NUMBER=1234567890"
	@echo "  make run PORT=8080 SPEED_USER=myuser SPEED_PASSWD=mypass"
