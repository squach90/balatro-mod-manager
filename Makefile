# Define OS-specific variables
ifeq ($(OS),Windows_NT)
	CRATE_DIR = src-tauri/lovely-injector/crates/lovely-win
	REMOVE_TARGET =
	SET_SKIP_BUILD_SCRIPT = set SKIP_BUILD_SCRIPT=1
else
	CRATE_DIR = src-tauri/lovely-injector/crates/lovely-mac
	REMOVE_TARGET = rm -f ../../target/release/liblovely.d*
	SET_SKIP_BUILD_SCRIPT = SKIP_BUILD_SCRIPT=1
	CLEAR_SCREEN = clear
	MACOS_TARGET = MACOSX_DEPLOYMENT_TARGET=11.0
endif

# Debug target
debug:
	@$(CLEAR_SCREEN)
	@echo "Building debug version in $(CRATE_DIR)..."
	@$(REMOVE_TARGET)
	@cd $(CRATE_DIR) && $(SET_SKIP_BUILD_SCRIPT) cargo build --release
	@cargo tauri dev

# Release target
release:
	@$(CLEAR_SCREEN)
	@echo "Building release version in $(CRATE_DIR)..."
	@$(REMOVE_TARGET)
	@cd $(CRATE_DIR) && $(SET_SKIP_BUILD_SCRIPT) $(MACOS_TARGET) cargo build --release
	@cargo tauri build --verbose

# Clean target
clean:
	@echo "Cleaning all build files..."
	@cd ./src-tauri && cargo clean
	@cd $(CRATE_DIR) && cargo clean

