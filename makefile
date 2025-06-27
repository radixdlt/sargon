bump_and_commit:
	./scripts/bump_and_commit.sh

bump:
	./scripts/bump_no_commit.sh

bump_to:
	./scripts/bump_to.sh $(version)

build_ios:
	./scripts/ios/build-sargon.sh

build_mac:
	./scripts/ios/build-sargon.sh --maconly

test_mac:
	./scripts/ios/test.sh

test_mac_build:
	./scripts/ios/test.sh --build

cov:
	RUST_LOG=none cargo tarpaulin  --skip-clean

clean:
	cargo clean
	rm cobertura.xml 2> /dev/null || true
	rm -rf .build 2> /dev/null || true
	rm -rf examples/.build 2> /dev/null || true
	rm -rf examples/android/.build 2> /dev/null || true
	rm -rf jvm/sargon-android/build 2> /dev/null || true
	rm -rf jvm/.build 2> /dev/null || true
	rm -rf crates/.build 2> /dev/null || true
	rm -rf target 2> /dev/null || true
