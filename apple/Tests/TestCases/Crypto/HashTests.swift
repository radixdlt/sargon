final class HashTests: TestCase {
	func test_blake2b() {
		func doTest(_ msg: String, expected: Exactly32Bytes) {
			let sut = Data(msg.utf8).hash()
			XCTAssertEqual(sut, expected)
		}
		doTest("Hello Radix", expected: "48f1bd08444b5e713db9e14caac2faae71836786ac94d645b00679728202a935")
		doTest("Radix... just imagine", expected: "679dfbbda16d3f9669da8552ab6594d2b0446d03d96c076a0ec9dc550ea41fad")
	}
}
