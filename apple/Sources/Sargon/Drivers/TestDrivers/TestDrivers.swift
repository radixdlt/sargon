import Foundation
import SargonUniFFI

#if DEBUG

extension BIOS {
	public static func insecure(
		bundle: Bundle = .main,
		userDefaultsSuite: String = "test"
	) -> BIOS {
		BIOS(
			bundle: bundle,
			userDefaultsSuite: userDefaultsSuite,
			unsafeStorageKeyMapping: [:],
			secureStorageDriver: Insecure︕！TestOnly︕！Ephemeral︕！SecureStorage(
				keychainService: "test"
			),
			arculuCSDKDriver: ArculusCsdkDriverImpl(noPointer: .init()),
			nftTagDriver: NfcTagDriverImpl(noPointer: .init())
		)
	}
}

#endif
