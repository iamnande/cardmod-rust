.PHONY: default help

default: help
help: ## help: display make targets
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m make %-20s -> %s\n\033[0m", $$1, $$2}'

# make: app info
APP_NAME    := cardmod
APP_WORKDIR := $(shell pwd)
APP_LOG_FMT := `/bin/date "+%Y-%m-%d %H:%M:%S %z [$(APP_NAME)]"`

# --------------------------------------------------
# Build Targets
# --------------------------------------------------
BIN_DIR   := $(APP_WORKDIR)/bin
BUILD_DIR := $(APP_WORKDIR)/target

.PHONY: build-clean
build-clean: ## build: clean build workspace
	@echo $(APP_LOG_FMT) "cleaning build workspace"
	@rm -rf $(BUILD_DIR) $(BIN_DIR)

.PHONY: build-binary
build-binary: build-clean ## build: build release binary file
	@echo $(APP_LOG_FMT) "building release binary"
	@cargo b --release
	@mkdir $(BIN_DIR) \
		&& cp $(BUILD_DIR)/release/$(APP_NAME) $(BIN_DIR)

# --------------------------------------------------
# Test Targets
# --------------------------------------------------

.PHONY: test-lint
test-lint: ## test: build debug binary file
	@echo $(APP_LOG_FMT) "building debug binary"
	@cargo c