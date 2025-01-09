import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class ThrowingHostInteractorTests: TestCase {
	typealias SUT = ThrowingHostInteractor

	func testDeriveKeysThrows() async throws {
		do {
			let _ = try await SUT.shared.deriveKeys(
				request: SargonUniFFI.KeyDerivationRequest(derivationPurpose: .creatingNewAccount, perFactorSource: [])
			)
		} catch {
			XCTAssertEqual(error as? CommonError, CommonError.SigningRejected)
		}
	}

	func testSignAuthThrows() async throws {
		do {
			let _ = try await SUT.shared.signAuth(
				request: SargonUniFFI.SignRequestOfAuthIntent(factorSourceKind: .device, perFactorSource: [])
			)
		} catch {
			XCTAssertEqual(error as? CommonError, CommonError.SigningRejected)
		}
	}

	func testSignTransactionsThrows() async throws {
		do {
			let _ = try await SUT.shared.signTransactions(
				request: SargonUniFFI.SignRequestOfTransactionIntent(factorSourceKind: .device, perFactorSource: [])
			)
		} catch {
			XCTAssertEqual(error as? CommonError, CommonError.SigningRejected)
		}
	}

	func testSignSubintentsThrows() async throws {
		do {
			let _ = try await SUT.shared.signSubintents(
				request: SargonUniFFI.SignRequestOfSubintent(factorSourceKind: .device, perFactorSource: [])
			)
		} catch {
			XCTAssertEqual(error as? CommonError, CommonError.SigningRejected)
		}
	}
}
