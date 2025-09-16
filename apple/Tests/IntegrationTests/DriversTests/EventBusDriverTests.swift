import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

// MARK: - EventBusDriverTests
// class EventBusDriverTests: DriverTest<EventBus> {
//	func test() async throws {
//		let sut = SUT()
//
//		let expectedEvents = [EventKind]([.booted, .profileSaved, .profileSaved, .factorSourceUpdated, .accountAdded, .profileSaved])
//		let task = Task {
//			var notifications = Set<EventNotification>()
//			for await notification in await sut.notifications().prefix(expectedEvents.count) {
//				notifications.insert(notification)
//			}
//			return notifications
//		}
//
//		let bios = BIOS(drivers: .withEventBus(sut))
//		let os = await TestOS(bios: bios)
//		try await os.os.newWallet(shouldPreDeriveInstances: false)
//
//		try await os.createAccount()
//		let notifications = await task.value
//		XCTAssertEqual(Set(notifications.map(\.event.kind)), Set(expectedEvents))
//	}
// }

extension HostInfoDriver where Self == AppleHostInfoDriver {
	static var shared: Self {
		AppleHostInfoDriver(appVersion: "0.0.0")
	}
}

#if DEBUG
extension SecureStorageDriver where Self == Insecure︕！TestOnly︕！Ephemeral︕！SecureStorage {
	static var shared: Self {
		Self(keychainService: "test")
	}
}

extension Drivers {
	static func withNetworking(_ networking: some NetworkingDriver) -> Drivers {
		Drivers(
			networking: networking,
			secureStorage: .shared,
			entropyProvider: .shared,
			hostInfo: .shared,
			logging: .shared,
			eventBus: .shared,
			fileSystem: .shared,
			unsafeStorage: .shared,
			profileStateChangeDriver: .shared,
			arculusCsdkDriver: ArculusCsdkDriverImpl(noPointer: .init()),
			nfcTagDriver: NfcTagDriverImpl(noPointer: .init())
		)
	}

	static func withSecureStorage(_ secureStorage: some SecureStorageDriver) -> Drivers {
		Drivers(
			networking: .shared,
			secureStorage: secureStorage,
			entropyProvider: .shared,
			hostInfo: .shared,
			logging: .shared,
			eventBus: .shared,
			fileSystem: .shared,
			unsafeStorage: .shared,
			profileStateChangeDriver: .shared,
			arculusCsdkDriver: ArculusCsdkDriverImpl(noPointer: .init()),
			nfcTagDriver: NfcTagDriverImpl(noPointer: .init())
		)
	}

	static func withEntropyProvider(_ entropyProvider: some EntropyProviderDriver) -> Drivers {
		Drivers(
			networking: .shared,
			secureStorage: .shared,
			entropyProvider: entropyProvider,
			hostInfo: .shared,
			logging: .shared,
			eventBus: .shared,
			fileSystem: .shared,
			unsafeStorage: .shared,
			profileStateChangeDriver: .shared,
			arculusCsdkDriver: ArculusCsdkDriverImpl(noPointer: .init()),
			nfcTagDriver: NfcTagDriverImpl(noPointer: .init())
		)
	}

	static func withHostInfo(_ hostInfo: some HostInfoDriver) -> Drivers {
		Drivers(
			networking: .shared,
			secureStorage: .shared,
			entropyProvider: .shared,
			hostInfo: hostInfo,
			logging: .shared,
			eventBus: .shared,
			fileSystem: .shared,
			unsafeStorage: .shared,
			profileStateChangeDriver: .shared,
			arculusCsdkDriver: ArculusCsdkDriverImpl(noPointer: .init()),
			nfcTagDriver: NfcTagDriverImpl(noPointer: .init())
		)
	}

	static func withLogging(_ logging: some LoggingDriver) -> Drivers {
		Drivers(
			networking: .shared,
			secureStorage: .shared,
			entropyProvider: .shared,
			hostInfo: .shared,
			logging: logging,
			eventBus: .shared,
			fileSystem: .shared,
			unsafeStorage: .shared,
			profileStateChangeDriver: .shared,
			arculusCsdkDriver: ArculusCsdkDriverImpl(noPointer: .init()),
			nfcTagDriver: NfcTagDriverImpl(noPointer: .init())
		)
	}

	static func withEventBus(_ eventBus: some EventBusDriver) -> Drivers {
		Drivers(
			networking: .shared,
			secureStorage: .shared,
			entropyProvider: .shared,
			hostInfo: .shared,
			logging: .shared,
			eventBus: eventBus,
			fileSystem: .shared,
			unsafeStorage: .shared,
			profileStateChangeDriver: .shared,
			arculusCsdkDriver: ArculusCsdkDriverImpl(noPointer: .init()),
			nfcTagDriver: NfcTagDriverImpl(noPointer: .init())
		)
	}

	static func withFileSystem(_ fileSystem: some FileSystemDriver) -> Drivers {
		Drivers(
			networking: .shared,
			secureStorage: .shared,
			entropyProvider: .shared,
			hostInfo: .shared,
			logging: .shared,
			eventBus: .shared,
			fileSystem: fileSystem,
			unsafeStorage: .shared,
			profileStateChangeDriver: .shared,
			arculusCsdkDriver: ArculusCsdkDriverImpl(noPointer: .init()),
			nfcTagDriver: NfcTagDriverImpl(noPointer: .init())
		)
	}

	static func withUnsafeStorage(_ unsafeStorage: some UnsafeStorageDriver) -> Drivers {
		Drivers(
			networking: .shared,
			secureStorage: .shared,
			entropyProvider: .shared,
			hostInfo: .shared,
			logging: .shared,
			eventBus: .shared,
			fileSystem: .shared,
			unsafeStorage: unsafeStorage,
			profileStateChangeDriver: .shared,
			arculusCsdkDriver: ArculusCsdkDriverImpl(noPointer: .init()),
			nfcTagDriver: NfcTagDriverImpl(noPointer: .init())
		)
	}
}
#endif
