# Define OS-specific variables
ifeq ($(OS),Windows_NT)
	REMOVE_TARGET = @echo.
	SET_SKIP_BUILD_SCRIPT = set SKIP_BUILD_SCRIPT=1
	CLEAR_SCREEN = cls
	WINDOWS_TARGET = x86_64-pc-windows-msvc
else
	SET_SKIP_BUILD_SCRIPT = SKIP_BUILD_SCRIPT=1
	CLEAR_SCREEN = clear
	MACOS_TARGET = MACOSX_DEPLOYMENT_TARGET=11.0
	MACOS_ARCH = universal-apple-darwin
endif

# Debug target
debug:
	@$(CLEAR_SCREEN)
	@$(REMOVE_TARGET)
	@cargo tauri dev

# Platform-specific release targets
release-macos:
	@$(CLEAR_SCREEN)
	@$(REMOVE_TARGET)
	@$(MACOS_TARGET) cargo tauri build --target $(MACOS_ARCH) --verbose

release-windows:
	@$(CLEAR_SCREEN)
	@$(REMOVE_TARGET)
	@cargo tauri build --target $(WINDOWS_TARGET) --verbose

# Alias default release to current platform
release: 
	@$(CLEAR_SCREEN)
	@$(REMOVE_TARGET)
ifeq ($(OS),Windows_NT)
	@cargo tauri build --target $(WINDOWS_TARGET) --verbose
else
	@$(MACOS_TARGET) cargo tauri build --target $(MACOS_ARCH) --verbose
endif

# Clean target
clean:
	@echo "Cleaning all build files..."
	@cd ./src-tauri && cargo clean

