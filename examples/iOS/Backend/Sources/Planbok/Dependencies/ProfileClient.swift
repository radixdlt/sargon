//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-05-05.
//

import Foundation
import Sargon
import DependenciesMacros

@DependencyClient
public struct ProfileClient: Sendable {
	public typealias ActiveProfile = @Sendable () -> Profile
	public typealias DeleteProfileAndMnemonicsThenCreateNew = @Sendable () async throws -> Void
	public typealias ImportProfile = @Sendable (Profile) async throws -> Void
	public typealias DecryptEncryptedProfile = @Sendable (_ encrypted: Data, _ password: String) throws -> Profile
	
	public typealias EmulateFreshInstallOfAppThenRestart = @Sendable () async throws -> Void

	public var activeProfile: ActiveProfile
	public var deleteProfileAndMnemonicsThenCreateNew: DeleteProfileAndMnemonicsThenCreateNew
	public var importProfile: ImportProfile
	public var decryptEncryptedProfile: DecryptEncryptedProfile
	public var emulateFreshInstallOfAppThenRestart: EmulateFreshInstallOfAppThenRestart
}

extension ProfileClient: DependencyKey {
	public static let liveValue = Self.live(os: SargonOS.shared)
	public static func live(os: SargonOS) -> Self {
		return Self(
			activeProfile: {
				try! os.profile()
			},
			deleteProfileAndMnemonicsThenCreateNew: {
                try await os.deleteWallet()
			},
			importProfile: {
				try await os.importProfile(profile: $0)
			},
			decryptEncryptedProfile: {
				try Profile(encrypted: $0, decryptionPassword: $1)
			},
			emulateFreshInstallOfAppThenRestart: {
                log.warning("TODO Migrate `emulateFreshInstallOfAppThenRestart`, not in Sargon anymore.")
                try await os.deleteWallet()
			}
		)
	}
}
