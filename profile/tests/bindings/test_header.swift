import radix_wallet_kit

extension Header {
	public static let placeholder = newHeaderPlaceholder()
	public static let placeholderOther = newHeaderPlaceholderOther()
}

func test_equatable() throws {
	let a = Header.placeholder
	let b = Header.placeholderOther
	assert(
		a == Header.placeholder
	)
	assert(a != b)
	assert(b == Header.placeholderOther)
}

func test_hashable() throws {
	let a = Header.placeholder
	let b = Header.placeholderOther
	assert(Set([a, a]).count == 1)
	assert(Set([b, b]).count == 1)
	assert(Set([a, b, b, a]).count == 2)
}

func test() throws {
	try test_equatable()
	try test_hashable()
}


try! test()
