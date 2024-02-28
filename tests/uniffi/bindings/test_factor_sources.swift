import Sargon

public typealias SUT = [FactorSource]
extension SUT {
	public static let sample: Self = newFactorSourcesSample()
	public static let sampleOther: Self = newFactorSourcesSampleOther()
}

func test_equatable() throws {
	let a = SUT.sample
	let b = SUT.sampleOther
	assert(
		a == SUT.sample
	)
	assert(a != b)
	assert(b == SUT.sampleOther)
}

func test_hashable() throws {
	let a = SUT.sample
	let b = SUT.sampleOther
	assert(Set([a, a]).count == 1)
	assert(Set([b, b]).count == 1)
	assert(Set([a, b, b, a]).count == 2)
}

func test() throws {
	try test_equatable()
	try test_hashable()
}

try! test()
