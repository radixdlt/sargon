import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

extension TestOS {
	public convenience init() async {
		await self.init(
			bios: BIOS(
				drivers: Drivers(
					networking: .shared,
					secureStorage: Insecure︕！TestOnly︕！Ephemeral︕！SecureStorage(
						keychainService: UUID().uuidString
					),
					entropyProvider: .shared,
					hostInfo: .shared,
					logging: .shared,
					eventBus: EventBus(),
					fileSystem: .shared,
					unsafeStorage: UnsafeStorage(
						userDefaults: .init(
							suiteName: UUID().uuidString
						)!
					),
					profileStateChangeDriver: .shared
				)
			)
		)
	}
}
