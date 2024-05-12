import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

class EventBusDriverTests: DriverTest<EventBus> {
	
	func test() async throws {
		let sut = SUT()

		
		let task = Task {
			var notifications = Set<EventNotification>()
			for await notification in await sut.notifications().prefix(3) {
				notifications.insert(notification)
			}
			return notifications
		}
		
		let bios = BIOS(drivers: .withEventBus(sut))
		let os = try await TestOS(bios: bios)
		try await os.createAccount()
		let notifications = await task.value
		XCTAssertEqual(Set(notifications.map(\.event.kind)), Set([.booted, .profileSaved, .addedAccount]))
	}
	
}


extension SecureStorageDriver where Self == FAKE_SecureStorage_FAKE {
	static var shared: Self {
		FAKE_SecureStorage_FAKE(keychainService: "test")
	}
}

extension HostInfoDriver where Self == HostInfo {
	static var shared: Self {
		HostInfo(appVersion: "0.0.0")
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
			unsafeStorage: .shared
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
			unsafeStorage: .shared
		
		)
	}
	
	static func withSecureStorage(_ entropyProvider: some EntropyProviderDriver) -> Drivers {
		Drivers(
			networking: .shared,
			secureStorage: .shared,
			entropyProvider: entropyProvider,
			hostInfo: .shared,
			logging: .shared,
			eventBus: .shared,
			fileSystem: .shared,
			unsafeStorage: .shared
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
			unsafeStorage: .shared
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
			unsafeStorage: .shared
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
			unsafeStorage: .shared
		)
	}
	
	static func withEventBus(_ fileSystem: some FileSystemDriver) -> Drivers {
		Drivers(
			networking: .shared,
			secureStorage: .shared,
			entropyProvider: .shared,
			hostInfo: .shared,
			logging: .shared,
			eventBus: .shared,
			fileSystem: fileSystem,
			unsafeStorage: .shared
		)
	}
	
	static func withEventBus(_ unsafeStorage: some UnsafeStorageDriver) -> Drivers {
		Drivers(
			networking: .shared,
			secureStorage: .shared,
			entropyProvider: .shared,
			hostInfo: .shared,
			logging: .shared,
			eventBus: .shared,
			fileSystem: .shared,
			unsafeStorage: unsafeStorage
		)
	}
}
