import DependenciesMacros
import Foundation
import Sargon

// MARK: - MnemonicClient
@DependencyClient
public struct MnemonicClient: Sendable {
	public typealias LoadMnemonic = @Sendable (FactorSourceIDFromHash) async throws -> PrivateHierarchicalDeterministicFactorSource
	public typealias GenerateNewMnemonic = @Sendable (BIP39WordCount) -> Mnemonic
	public var loadMnemonic: LoadMnemonic
	public var generateNewMnemonic: GenerateNewMnemonic
}

// MARK: DependencyKey
extension MnemonicClient: DependencyKey {
	public static let liveValue = Self.live(os: SargonOS.shared)
	public static func live(os: SargonOS) -> Self {
		Self(
			loadMnemonic: { id in
				try await os.loadPrivateDeviceFactorSourceById(id: id)
			},
			generateNewMnemonic: { wordCount in
				Mnemonic(wordCount: wordCount, language: .english)
			}
		)
	}
}
