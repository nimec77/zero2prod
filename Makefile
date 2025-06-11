# Variables
CARGO := cargo
EXECUTABLE := my_rust_project
PREFIX := $(HOME)/.local

# Default target
all: build

# Build the project in release mode
build:
	$(CARGO) build --release

# Run the project
r:
	$(CARGO) run | bunyan

# Run tests
t:
	./scripts/run-test.sh $(ARGS)

ts:
	./scripts/run-test.sh --simple

# Format the code
f:
	$(CARGO) fmt

fc:
	$(CARGO) fmt --check

# Check for errors without building
c:
	$(CARGO) check

# Install the binary
install:
	cp target/release/$(EXECUTABLE) $(PREFIX)/bin

# Generate documentation
doc:
	$(CARGO) doc

# Clean build artifacts
clean:
	$(CARGO) clean

# Show help message
help:
	@echo "Available targets:"
	@grep -E '^[a-zA-Z_-]+:.*##' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*## "}; {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}'

.PHONY: all build run test fmt check install doc clean help
