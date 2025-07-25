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
			secureStorageDriver: secureStorageDriver,
			arculuCSDKDriver: ArculusCsdkDriverImpl(noPointer: .init()),
			nftTagDriver: NfcTagDriverImpl(noPointer: .init())
		)
	}
}

public final class TestOS {
	public let os: SargonOS

	public init(os: SargonOS) {
		self.os = os
	}

	public convenience init(bios: BIOS) async {
		let os = try! await SargonOS.boot(
			bios: bios,
			interactor: ThrowingHostInteractor.shared
		)
		self.init(os: os)
	}
}

extension TestOS: SargonOSProtocol {}

#endif // DEBUG
