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
	enum ProfileState {
		case ephemeral(Profile, mnemonic: Mnemonic)
		case persisted(Profile)
		func get() -> Profile {
			switch self {
			case let .ephemeral(profile, mnemonic: _):
				return profile
			case let .persisted(profile):
				return profile
			}
		}
		func mainBDFS(secureStorageDriver: SecureStorageDriver) async throws -> Mnemonic {
			switch self {
			case let .ephemeral(_, mnemonic):
				mnemonic
			case .persisted:
				try await secureStorageDriver.loadMainBDFSMnemonic()
			}
		}
		static func makeEphemeral() -> Self {
			.ephemeral(Profile(id: -1), mnemonic: .generate())
		}
	}
	private var profileState: ProfileState = .makeEphemeral()
	private let drivers: Drivers
	init(drivers: Drivers) {
		self.drivers = drivers
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
	
	func bootstrap() async -> Status {
		switch self.profileState {
		
		case .persisted:
			fatalError("should not call bootstrap when loaded.")
		
		case let .ephemeral(ephemeralProfile, unsavedMnemonic):
			func saveEphemeral() async -> Status {
			
				do {
					try await drivers.secureStorage.saveProfile(ephemeralProfile)
				} catch {
					return .bootstrapFailed(.failedToSaveEphemeralProfile)
				}
				
				do {
					try await drivers.secureStorage.saveMnemonic(unsavedMnemonic)
				} catch {
					return .bootstrapFailed(.failedToSaveEphemeralMnemonic)
				}
				
				self.profileState = .persisted(ephemeralProfile)
				return .savedEphemeral
			}
			do {
				if
					let json = try await drivers.secureStorage.loadProfile(),
					let profile = Profile(json: json)
				{
					self.profileState = .persisted(profile)
					return .loadedFromStorage
				} else {
					return await saveEphemeral()
				}
			} catch {
				return .bootstrapFailed(.failedToLoad)
			}
			
		}
		
	}
	
}

extension Drivers {
	public static let shared = Drivers(
		networking: .shared,
		secureStorage: .shared
	)
}

extension OS {
	public static let shared = OS(drivers: .shared)
}


struct SplashReducer {
	enum Action {
		case appear
		case delegate(DelegateAction)
		enum DelegateAction {
			enum Outcome {
				case failure(OS.Status.BootstrapFailed)
				case success(isProfileNew: Bool)
			}
			case bootstraped(Outcome)
		}
	}
	struct State {}
	
	func reduce(action: Action, state: State) async -> Action? {
		switch action {
		case .appear:
			switch await OS.shared.bootstrap() {
			case let .bootstrapFailed(failure):
				.delegate(.bootstraped(.failure(failure)))
			case .loadedFromStorage:
				.delegate(.bootstraped(.success(isProfileNew: false)))
		    case .savedEphemeral:
				.delegate(.bootstraped(.success(isProfileNew: true)))
			}
		case .delegate:
			nil
		}
	}
}
