import Sargon

final class NonFungibleLocalIDTests: IdentifiableByStringProtocolTest<NonFungibleLocalID> {
	
    // MARK: LocalID String
    func test_valid_local_id_string_from_string() {
		XCTAssertEqual(try SUT.init("<foo>"), SUT.str(value: "foo"))
    }
    
    func test_valid_local_id_string_from_integer() {
		XCTAssertEqual(try SUT.init("#666#"), SUT.integer(value: 666))
    }
    
    func test_valid_local_id_string_from_ruid() {
		XCTAssertEqual(try SUT.init("{deaddeaddeaddead-deaddeaddeaddead-deaddeaddeaddead-deaddeaddeaddead}"), SUT.ruid(value: .sample))
    }
    
    func test_valid_local_id_string_from_bytes() {
		XCTAssertEqual(try SUT.init("[acedacedacedacedacedacedacedacedacedacedacedacedacedacedacedaced]"), SUT.bytes(value: NonEmptyMax64Bytes(bagOfBytes: Data.sampleAced)))
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
			try SUT.stringID("x").description,
			"<x>"
		)
	}
	
	func test_init_string_fails_when_given_user_facing_string() throws {
		XCTAssertThrowsError(try SUT.init("x"))
	}
	
	func test_init_string_succeeds_when_given_raw_string() throws {
		XCTAssertNoThrow(try SUT.init("<x>"))
	}
	
	func test_from_stringID_fails_when_given_raw_string() throws {
		XCTAssertThrowsError(try SUT.stringID("<x>"))
	}
	
	func test_from_stringID_succeeds_when_given_user_facing_string() throws {
		XCTAssertNoThrow(try SUT.stringID("x"))
	}
	
	func test_string_valid_max_length() {
		let s = String(repeating: "z", count: 64)
		XCTAssertEqual(
			try SUT.stringID(s).description,
			"<\(s)>"
		)
	}
	
	func test_string_invalid_too_long() {
		XCTAssertThrowsError(try SUT.stringID("much2longmuch2longmuch2longmuch2longmuch2longmuch2longmuch2longmuch2long"))
	}
	
	func test_string_invalid_forbidden_chars() {
		XCTAssertThrowsError(try SUT.stringID("#$^"))
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
    
    func test_to_user_facing_string() {
        XCTAssertNoDifference(SUT.sampleOther.toUserFacingString(), "foobar")
    }
    
    func test_formatted() {
        XCTAssertNoDifference(SUT.sampleOther.formatted(.raw), "<foobar>")
        XCTAssertNoDifference(SUT.sampleOther.formatted(.default), "foobar")
    }
}
