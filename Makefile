GO_DIR = goCode/goApp
RUST_DIR = rustCode
LIBS_DIR = goCode/libs

# Linux specifics
RUST_LIB_LINUX = $(RUST_DIR)/target/release/librpc.so

# Windows specifics
RUST_LIB_WINDOWS_DLL = $(RUST_DIR)/target/release/rpc.dll
RUST_LIB_WINDOWS_LIB = $(RUST_DIR)/target/release/rpc.dll.lib

# build Rust library
RUST_BUILD_CMD = cargo build --release

RUST_TEST_CMD = cargo test

allLinux: build_rust copy_libs_linux run_go

allWindows: build_rust copy_libs_windows run_go

build_rust:
	@echo "Building Rust library for Linux..."
	cd $(RUST_DIR) && $(RUST_BUILD_CMD)

copy_libs_linux:
	@echo "Copying librpc.so to Go libs folder (Linux)..."
	cp $(RUST_LIB_LINUX) $(LIBS_DIR)

copy_libs_windows:
	@echo "Copying rpc.dll and rpc.dll.lib to Go libs folder (Windows)..."
	cp $(RUST_LIB_WINDOWS_DLL) $(LIBS_DIR)/rpc.dll
	cp $(RUST_LIB_WINDOWS_LIB) $(LIBS_DIR)/librpc.a  # Renaming .lib to .a

run_go:
	@echo "Running Go application..."
	cd $(GO_DIR) && go run main.go

run_rust:
	@echo "Running Rust code..."
	cd $(RUST_DIR) && cargo run

test_rust:
	@echo "Running Rust unit tests..."
	cd $(RUST_DIR) && $(RUST_TEST_CMD)

clean:
	@echo "Cleaning up..."
	rm -rf $(RUST_DIR)/target
	rm -f $(LIBS_DIR)/librpc.so
	rm -f $(LIBS_DIR)/rpc.dll
	rm -f $(LIBS_DIR)/librpc.a
