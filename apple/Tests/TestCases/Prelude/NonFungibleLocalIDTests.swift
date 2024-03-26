final class NonFungibleLocalIDTests: Test<NonFungibleLocalID> {
	
    // MARK: LocalID String
    func test_valid_local_id_string_from_string() {
        XCTAssertEqual(try SUT(localId: "<foo>"), SUT.str(value: "foo"))
    }
    
    func test_valid_local_id_string_from_integer() {
        XCTAssertEqual(try SUT(localId: "#666#"), SUT.integer(value: 666))
    }
    
    func test_valid_local_id_string_from_ruid() {
        XCTAssertEqual(try SUT(localId: "{deaddeaddeaddead-deaddeaddeaddead-deaddeaddeaddead-deaddeaddeaddead}"), SUT.ruid(value: .sample))
    }
    
    func test_valid_local_id_string_from_bytes() {
        XCTAssertEqual(try SUT(localId: "[acedacedacedacedacedacedacedacedacedacedacedacedacedacedacedaced]"), SUT.bytes(value: NonEmptyMax64Bytes(bagOfBytes: Data.sampleAced)))
    }
    
	// MARK: Integer
	func test_integer_valid() {
		XCTAssertEqual(
			SUT(integer: 42).description,
			"#42#"
		)
		XCTAssertEqual(
			SUT(integer: 12345678),
			12345678 as SUT // ExpressibleByIntegerliteral
		)
	}
	
	// MARK: String
	func test_string_valid_short() {
		XCTAssertEqual(
			try SUT(string: "x").description,
			"<x>"
		)
		XCTAssertEqual(
			try SUT(string: "x"),
			"x" as SUT // ExpressibleByStringLiteral
		)
	}
	
	func test_string_valid_max_length() {
		let s = String(repeating: "z", count: 64)
		XCTAssertEqual(
			try SUT(string: s).description,
			"<\(s)>"
		)
	}
	
	func test_string_invalid_too_long() {
		XCTAssertThrowsError(try SUT(string: "much2longmuch2longmuch2longmuch2longmuch2longmuch2longmuch2longmuch2long"))
	}
	
	func test_string_invalid_forbidden_chars() {
		XCTAssertThrowsError(try SUT(string: "#$^"))
	}
	
	// MARK: Bytes
	func test_bytes_valid_short() {
		XCTAssertEqual(
			try SUT(bytes: [0xde, 0xad, 0xbe, 0xef]).description,
			"[deadbeef]"
		)
		XCTAssertEqual(
			try SUT(bytes: [0xde, 0xad, 0xbe, 0xef]),
			[0xde, 0xad, 0xbe, 0xef] as SUT // ExpressibleByArrayLiteral
		)
	}
	
	func test_bytes_valid_max_len() {
		XCTAssertEqual(
			try SUT(bytes: Data(repeating: 0xab, count: 64)).description,
			"[abababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababab]"
		)
	}
	
	func test_bytes_invalid_empty() {
		XCTAssertThrowsError(try SUT(bytes: Data([])))
	}
	
	func test_bytes_invalid_too_long() {
		XCTAssertThrowsError(try SUT(bytes: Data(repeating: 0xff, count: 128)))
	}
	
	// MARK: RUID
	func test_ruid() {
		XCTAssertEqual(
			try SUT(ruid: Data.sampleAced).description,
			"{acedacedacedaced-acedacedacedaced-acedacedacedaced-acedacedacedaced}"
		)
	}
	
	func test_ruid_invalid_empty() {
		XCTAssertThrowsError(try SUT(ruid: Data([])))
	}
	
	func test_ruid_invalid_too_short() {
		XCTAssertThrowsError(try SUT(ruid: Data(repeating: 0xff, count: 16)))
	}
	
	func test_ruid_invalid_too_long() {
		XCTAssertThrowsError(try SUT(ruid: Data(repeating: 0xff, count: 128)))
	}
}
