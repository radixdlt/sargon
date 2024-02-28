import Sargon

extension Header {
	public static let sample = newHeaderSample()
	public static let sampleOther = newHeaderSampleOther()
}

func test_equatable() throws {
	let a = Header.sample
	let b = Header.sampleOther
	assert(
		a == Header.sample
	)
	assert(a != b)
	assert(b == Header.sampleOther)
}

func test_hashable() throws {
	let a = Header.sample
	let b = Header.sampleOther
	assert(Set([a, a]).count == 1)
	assert(Set([b, b]).count == 1)
	assert(Set([a, b, b, a]).count == 2)
}

func test() throws {
	try test_equatable()
	try test_hashable()
}

try! test()
