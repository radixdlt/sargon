@testable import Sargon

final class TransactionIntentTests: Test<TransactionIntent> {
	func test_hash() {
		XCTAssertEqual(SUT.sample.hash().description, "txid_rdx12nnrygyt3p5v5pft5e3vu93v38qp5k7fh9v59kd6vtu8506880nq5vsxx6")
	}
	
	func test_compile() {
		XCTAssertEqual(SUT.sample.compile().hex, "4d220104210707010a872c0100000000000a912c01000000000009092f2400220101200720ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf010108000020220441038000d1be9c042f627d98a01383987916d43cf439631ca1d8c8076d6754ab263d0c086c6f636b5f6665652101850000fda0c42777080000000000000000000000000000000041038000d1be9c042f627d98a01383987916d43cf439631ca1d8c8076d6754ab263d0c087769746864726177210280005da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6850000443945309a7a48000000000000000000000000000000000280005da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6850000443945309a7a4800000000000000000000000000000041038000d1127918c16af09af521951adcf3a20ab2cc87c0e72e85814764853ce5e70c147472795f6465706f7369745f6f725f61626f72742102810000000022000020200022010121020c0a746578742f706c61696e2200010c0c48656c6c6f20526164697821")
	}
}
