bump_and_commit:
	./scripts/bump_and_commit.sh

bump:
	./scripts/bump_no_commit.sh

build_ios:
	./scripts/ios/build-sargon.sh

build_mac:
	./scripts/ios/build-sargon.sh --maconly

test_mac:
	./scripts/ios/test.sh

test_mac_build:
	./scripts/ios/test.sh --build

clean:
	cargo clean
	rm cobertura.xml 2> /dev/null || true
