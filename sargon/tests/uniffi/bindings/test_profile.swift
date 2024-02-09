import Sargon

extension Profile {
	fileprivate static let placeholder = newProfilePlaceholder()
	fileprivate static let placeholderOther = newProfilePlaceholderOther()
}

func test_equatable() throws {
	let p = Profile.placeholder
	let q = Profile.placeholderOther
	assert(
		p == Profile.placeholder
	)
	assert(p != q)
	assert(q == Profile.placeholderOther)
}

func test_hashable() throws {
	let a = Profile.placeholder
	let b = Profile.placeholderOther
	assert(Set([a, a]).count == 1)
	assert(Set([b, b]).count == 1)
	assert(Set([a, b, b, a]).count == 2)
}

func test() throws {
	try test_equatable()
	try test_hashable()
}


try! test()
