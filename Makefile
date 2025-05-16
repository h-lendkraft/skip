.PHONY: build run login search-mobile health clean stop logs all

# Configuration variables - edit these as needed
PORT ?= 3000
SPEED_USER ?= BACKESHMOORHTY
SPEED_PASSWD ?= backeshmoorhty123
SPEED_BASE_URL ?= https://visit.checkmycontent.site
API_BASE_URL ?= http://localhost:$(PORT)
# API_BASE_URL ?= https://apps.lendkraft.ai
# API_BASE_URL ?=https://135f-2409-40f4-3088-4301-7da3-70f4-e91-9eb9.ngrok-free.app

#
# Test commands - step by step testing
health:
	@echo "Testing health endpoint..."
	curl -s $(API_BASE_URL)/health | jq

login:
	@echo "Testing login functionality..."
	curl -X GET $(API_BASE_URL)/speed/login 

search-namedob:
	@echo "Testing name and DOB search..."
	@if [ -z "$(NAME)" ] || [ -z "$(DOB)" ] || [ -z "$(STATE)" ]; then \
		echo "Error: Please provide NAME, DOB, and STATE. Usage: make search-namedob NAME=\"John Doe\" DOB=\"01-01-1990\" STATE=33"; \
		exit 1; \
	fi; \
	echo "Searching for $$NAME born on $$DOB in state $$STATE..."; \
	curl -s -X POST $(API_BASE_URL)/speed/search/name-dob \
		-H "Content-Type: application/json" \
		-d "{\"pairs\":[{\"name\":\"$$NAME\",\"dob\":\"$$DOB\"}],\"state\":$(STATE)}" | jq

search-mobile:
	@if [ -z "$(NUMBERS)" ]; then \
		echo "Error: Please provide mobile numbers. Usage: make search NUMBERS=\"1111111111\" STATE=22 or make search NUMBERS=\"111111111 2222222222\"" STATE=11; \
		exit 1; \
	fi; \
	for number in $(NUMBERS); do \
		echo "Searching for $$number..."; \
		curl -s -X POST $(API_BASE_URL)/speed/search/mobile \
			-H "Content-Type: application/json" \
			-d "{\"numbers\":[\"$$number\"],\"state\":$(STATE)}" | jq; \
		echo "\n"; \
	done

search-aadhar:
	@if [ -z "$(AADHARS)" ] || [ -z "$(STATE)" ]; then \
		echo "Error: Please provide Aadhar numbers and state. Usage: make search-aadhar-batch AADHARS=\"123456789012 234567890123\" STATE=33"; \
		exit 1; \
	fi; \
	numbers=""; \
	for number in $(AADHARS); do \
		if [ -n "$$numbers" ]; then numbers="$$numbers,"; fi; \
		numbers="$$numbers\"$$number\""; \
	done; \
	echo "Searching for multiple Aadhar numbers in state $(STATE)..."; \
	curl -s -X POST $(API_BASE_URL)/speed/search/aadhar \
		-H "Content-Type: application/json" \
		-d "{\"numbers\":[$$numbers],\"state\":$(STATE)}" | jq

run:
	@echo "Starting the application on port $(PORT)..."
	RUST_LOG=info PORT=$(PORT) SPEED_USER=$(SPEED_USER) SPEED_PASSWD=$(SPEED_PASSWD) SPEED_BASE_URL=$(SPEED_BASE_URL) cargo run

