// The Swift Programming Language
// https://docs.swift.org/swift-book

import Foundation

public protocol NetworkingDriver: AnyObject {}
extension URLSession: NetworkingDriver {}
extension NetworkingDriver where Self == URLSession {
	public static var shared: Self { URLSession.shared }
}

public typealias JSON = Int

public protocol SecureStorageDriver: AnyObject {
	func loadProfile() async throws -> JSON?
	func loadMainBDFSMnemonic() async throws -> Mnemonic
	func saveProfile(_ profile: Profile) async throws -> Void
	func saveMnemonic(_ mnemonic: Mnemonic) async throws -> Void
}

struct Err: Error {}

public final class Keychain: SecureStorageDriver {
	private init() {}

	public func loadProfile() async throws -> JSON? { nil }
	public func loadMainBDFSMnemonic() async throws -> Mnemonic { throw Err() }
	public func saveProfile(_ profile: Profile) async throws -> Void {}
	public func saveMnemonic(_ mnemonic: Mnemonic) async throws -> Void {}
	
	public static let shared = Keychain()
}

extension SecureStorageDriver where Self == Keychain {
	public static var shared: Self { Keychain.shared }
}


public class Drivers {
	fileprivate let networking: NetworkingDriver
	fileprivate let secureStorage: SecureStorageDriver
	required init(
		networking: NetworkingDriver,
		secureStorage: SecureStorageDriver
	) {
		self.networking = networking
		self.secureStorage = secureStorage
	}
}

public struct Profile {
	let id: Int
	init(id: Int) {
		self.id = id
	}
	var hasAnyNetworks: Bool {
		false
	}
	init?(json: JSON) {
		self.init(id: json)
	}
}

public struct Mnemonic {
	let phrase: String
	static func generate() -> Self {
		.init(phrase: "zoo zoo")
	}
}

public class OS {
	
	fileprivate let profile: Profile
	private let drivers: Drivers
	
	init(drivers: Drivers) async throws {
		self.drivers = drivers
		
		func generateNew() async throws -> Profile {
			let newProfile = Profile(id: .random(in: 0...Int.max))
			let newBDFS = Mnemonic.generate()
			
			try await drivers.secureStorage.saveProfile(newProfile)
			try await drivers.secureStorage.saveMnemonic(newBDFS)
			return newProfile
		}
		
		if
			let json = try await drivers.secureStorage.loadProfile(),
			let loadedProfile = Profile(json: json)
		{
			self.profile = loadedProfile
		} else {
			self.profile = try await generateNew()
		}
		
	}
	
	enum Status {
		case loadedFromStorage
		case savedEphemeral
		enum BootstrapFailed {
			case failedToLoad
			case failedToSaveEphemeralProfile
			case failedToSaveEphemeralMnemonic
		}
		case bootstrapFailed(BootstrapFailed)
	}
	
	
	
}

extension Drivers {
	public static let shared = Drivers(
		networking: .shared,
		secureStorage: .shared
	)
}



struct SplashReducer {
	enum Action {
		case appear
		case delegate(DelegateAction)
		enum DelegateAction {
			enum Outcome {
				case success(isProfileNew: Bool)
				case failure
			}
			case done(outcome: Outcome)
		}
	}
	struct State {}
	
	func reduce(action: Action, state: State) async -> Action? {
		switch action {
		case .appear:
			do {
				let os = try  await OS(drivers: .shared)
				return .delegate(.done(outcome: .success(isProfileNew: os.profile.hasAnyNetworks)))
			} catch {
				return .delegate(.done(outcome: .failure))
			}
		case .delegate:
			return nil
		}
	}
}

struct MainReducer {
	
}
