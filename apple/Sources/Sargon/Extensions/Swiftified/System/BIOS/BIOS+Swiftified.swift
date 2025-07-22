import Foundation
import SargonUniFFI

public typealias BIOS = Bios

// MARK: - BIOS + @unchecked Sendable
extension BIOS: @unchecked Sendable {}

extension BIOS {
	public convenience init(
		bundle: Bundle,
		userDefaultsSuite: String,
		unsafeStorageKeyMapping: UnsafeStorageKeyMapping,
		secureStorageDriver: SecureStorageDriver,
        arculuCSDKDriver: ArculusCsdkDriver,
        nftTagDriver: NfcTagDriver
	) {
		let drivers = Drivers(
			bundle: bundle,
			userDefaultsSuite: userDefaultsSuite,
			unsafeStorageKeyMapping: unsafeStorageKeyMapping,
			secureStorageDriver: secureStorageDriver,
            arculuCSDKDriver: arculuCSDKDriver,
            nftTagDriver: nftTagDriver
		)
		// https://en.wikipedia.org/wiki/Power-on_self-test
		log.info("📬 BIOS POST (Power-On Self Test)")

		self.init(drivers: drivers)
	}
}
