LIB=librust_native.a

all: ios

ios:
	RUSTFLAGS="-Z embed-bitcode" cargo +ios-arm64 build --target aarch64-apple-ios --release --lib
	lipo -create -output $(LIB) target/aarch64-apple-ios/release/$(LIB)
