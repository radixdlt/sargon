// import CustomDump
// import Foundation
// import Sargon
// import SargonUniFFI
// import XCTest
//
// final class SecurityQuestionsFactorSourceTests: SpecificFactorSourceTest<SecurityQuestionsNotProductionReadyFactorSource> {
//	func test_id_of_security_questions() {
//		eachSample { sut in
//			XCTAssertEqual(sut.id.description, FactorSourceID.hash(value: sut.id).description)
//		}
//	}
//
//	func test_roundtrip() throws {
//		let mnemonic = Mnemonic.sampleSecurityQuestions
//		let qas = newSecurityNOTPRODUCTIONREADYQuestionsAndAnswersSample()
//		let sut = try SUT(
//			mnemonic: mnemonic,
//			questionsAndAnswers: qas
//		)
//		let decrypted = try sut.decrypt(questionsAndAnswers: qas)
//		XCTAssertEqual(decrypted, mnemonic)
//	}
//
//	func test_new() throws {
//		XCTAssertEqual(
//			try SUT(
//				mnemonic: .sampleSecurityQuestions,
//				questionsAndAnswers: newSecurityNOTPRODUCTIONREADYQuestionsAndAnswersSample()
//			).id,
//			SUT.sample.id
//		)
//	}
//
//	func test_as() {
//		eachSample { sut in
//			XCTAssertEqual(sut.asGeneral.asSecurityQuestions, sut)
//		}
//	}
//
//	func test_other_wrong() {
//		XCTAssertNil(SUT.extract(from: DeviceFactorSource.sample))
//	}
//
//	func test_factor_source_id_is_id() {
//		eachSample { sut in
//			XCTAssertEqual(sut.id.asGeneral, sut.factorSourceID)
//		}
//	}
//
//	func test_kind() {
//		eachSample { sut in
//			XCTAssertEqual(sut.factorSourceKind, .securityQuestions)
//		}
//	}
//
//	func test_as_factor_source_to_string() {
//		eachSample { sut in
//			XCTAssertEqual(sut.asGeneral.id.description, sut.id.description)
//		}
//	}
//
//	func test_as_general() {
//		eachSample { sut in
//			XCTAssertEqual(sut.asGeneral, FactorSource.securityQuestions(value: sut))
//		}
//	}
//
//	func test_source_that_supports_babylon() {
//		let sut = SUT.sample
//		XCTAssertTrue(sut.supportsBabylon)
//		XCTAssertFalse(sut.supportsOlympia)
//	}
//
//	func test_extract_wrong_throws() throws {
//		try eachSample { sut in
//			XCTAssertThrowsError(try sut.asGeneral.extract(as: DeviceFactorSource.self))
//		}
//	}
// }
