import Foundation
import SargonUniFFI

extension MnemonicWithPassphrase {
	public init(jsonData: some DataProtocol) throws {
		self = try newMnemonicWithPassphraseFromJsonBytes(
			jsonBytes: Data(jsonData)
		)
	}

	public func jsonData() -> Data {
		mnemonicWithPassphraseToJsonBytes(
			mnemonicWithPassphrase: self
		)
	}

	public func validate(
		publicKeys: some Collection<HierarchicalDeterministicPublicKey>
	) -> Bool {
		mnemonicWithPassphraseValidatePublicKeys(
			mnemonicWithPassphrase: self,
			hdKeys: Array(publicKeys)
		)
	}

	public func derivePublicKeys(
		paths: some Collection<some DerivationPathProtocol>
	) -> [HierarchicalDeterministicPublicKey] {
		mnemonicWithPassphraseDerivePublicKeys(
			mnemonicWithPassphrase: self,
			derivationPaths: paths.map(\.asDerivationPath)
		)
	}

	public func sign(
		hash: Hash,
		path: some DerivationPathProtocol
	) -> SignatureWithPublicKey {
		mnemonicWithPassphraseSign(
			mnemonicWithPassphrase: self,
			derivationPath: path.asDerivationPath,
			hashToSign: hash
		)
	}
}
