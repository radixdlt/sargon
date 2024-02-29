import Sargon

extension Profile {
	fileprivate static let sample = newProfileSample()
	fileprivate static let sampleOther = newProfileSampleOther()
}

func test_equatable() throws {
	let p = Profile.sample
	let q = Profile.sampleOther
	assert(
		p == Profile.sample
	)
	assert(p != q)
	assert(q == Profile.sampleOther)
}

func test_hashable() throws {
	let a = Profile.sample
	let b = Profile.sampleOther
	assert(Set([a, a]).count == 1)
	assert(Set([b, b]).count == 1)
	assert(Set([a, b, b, a]).count == 2)
}

func test() throws {
	try test_equatable()
	try test_hashable()
}

try! test()
