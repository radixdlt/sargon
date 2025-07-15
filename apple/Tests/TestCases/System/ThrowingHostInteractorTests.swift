import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class ThrowingHostInteractorTests: TestCase {
	typealias SUT = ThrowingHostInteractor

	func testDeriveKeysThrows() async throws {
		do {
			_ = try await SUT.shared.deriveKeys(
				request: SargonUniFFI.KeyDerivationRequest(derivationPurpose: .creatingNewAccount, perFactorSource: [])
			)
		} catch {
			XCTAssertEqual(error as? CommonError, CommonError.HostInteractionAborted)
		}
	}

	func testSignAuthThrows() async throws {
		do {
			_ = try await SUT.shared.signAuth(
				request: SargonUniFFI.SignRequestOfAuthIntent(factorSourceKind: .device, perFactorSource: [])
			)
		} catch {
			XCTAssertEqual(error as? CommonError, CommonError.HostInteractionAborted)
		}
	}

	func testSignTransactionsThrows() async throws {
		do {
			_ = try await SUT.shared.signTransactions(
				request: SargonUniFFI.SignRequestOfTransactionIntent(factorSourceKind: .device, perFactorSource: [])
			)
		} catch {
			XCTAssertEqual(error as? CommonError, CommonError.HostInteractionAborted)
		}
	}

	func testSignSubintentsThrows() async throws {
		do {
			_ = try await SUT.shared.signSubintents(
				request: SargonUniFFI.SignRequestOfSubintent(factorSourceKind: .device, perFactorSource: [])
			)
		} catch {
			XCTAssertEqual(error as? CommonError, CommonError.HostInteractionAborted)
		}
	}

	func testAuthoriseGetsRejected() async {
		let outcome = await SUT.shared.requestAuthorization(purpose: .creatingAccount)
		XCTAssertEqual(outcome, .rejected)
	}
}
