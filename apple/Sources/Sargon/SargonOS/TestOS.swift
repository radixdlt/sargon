import Foundation
import SargonUniFFI

#if DEBUG

extension BIOS {
	public static func test(
		bundle: Bundle = .main,
		userDefaultsSuite: String = "Test",
		unsafeStorageKeyMapping: UnsafeStorageKeyMapping = [:],
		secureStorageDriver: SecureStorageDriver
	) -> BIOS {
		BIOS(
			bundle: bundle,
			userDefaultsSuite: userDefaultsSuite,
			unsafeStorageKeyMapping: unsafeStorageKeyMapping,
			secureStorageDriver: secureStorageDriver
		)
	}
}

public final class TestOS {
	public let os: SargonOS

	public init(os: SargonOS) {
		self.os = os
	}

	public convenience init(bios: BIOS) async {
		let os = await SargonOS.boot(
			bios: bios,
			interactor: ThrowingHostInteractor.shared
		)
		self.init(os: os)
	}
}

extension TestOS: SargonOSProtocol {}

//// MARK: Private
////extension TestOS {
////	private func nextAccountName() -> DisplayName {
////		let index = (try? accountsForDisplayOnCurrentNetwork.count) ?? 0
////		return DisplayName(value: "Unnamed \(index)")
////	}
////}
//
//// MARK: Public
// extension TestOS {
//	@discardableResult
//	public func createAccount(
//		named name: String? = nil
//	) async throws -> Self {
//		let accountName = try name.map {
//			try DisplayName(
//				validating: $0
//			)
//		} ?? nextAccountName()
//
//		let _ = try await os.createAccountWithBDFS(
//			networkId: nil,
//			name: accountName
//		)
//		return self
//	}
//
//	@discardableResult
//	public func batchCreateAccounts(
//		count: UInt16,
//		namePrefix: DisplayName
//	) async throws -> Self {
//		let _ = try await os.batchCreateManyAccountsWithMainBdfsThenSaveOnce(count: count, networkId: currentNetworkID, namePrefix: namePrefix.value)
//		return self
//	}
// }

#endif // DEBUG
