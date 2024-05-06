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
	public typealias DeleteProfileAndMnemonicsThenCreateNew = @Sendable () async throws -> Void
	public typealias ImportProfile = @Sendable (Profile) async throws -> Void
	public typealias DecryptEncryptedProfile = @Sendable (_ encrypted: Data, _ password: String) throws -> Profile
	
	public typealias EmulateFreshInstallOfAppThenRestart = @Sendable () async throws -> Void

	public var deleteProfileAndMnemonicsThenCreateNew: DeleteProfileAndMnemonicsThenCreateNew
	public var importProfile: ImportProfile
	public var decryptEncryptedProfile: DecryptEncryptedProfile
	public var emulateFreshInstallOfAppThenRestart: EmulateFreshInstallOfAppThenRestart
}

extension ProfileClient: DependencyKey {
	public static let liveValue = Self.live(os: SargonOS.shared)
	public static func live(os: SargonOS) -> Self {
		return Self(
			deleteProfileAndMnemonicsThenCreateNew: {
				let _ = try await os.deleteProfileThenCreateNewWithBdfs()
			},
			importProfile: {
				try await os.importProfile(profile: $0)
			},
			decryptEncryptedProfile: {
				try Profile(encrypted: $0, decryptionPassword: $1)
			},
			emulateFreshInstallOfAppThenRestart: {
				try await os.emulateFreshInstall()
			}
		)
	}
}
