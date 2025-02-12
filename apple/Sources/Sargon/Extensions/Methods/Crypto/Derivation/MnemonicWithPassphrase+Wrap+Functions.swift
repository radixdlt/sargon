import Foundation
import SargonUniFFI

extension MnemonicWithPassphrase {
	public init(mnemonic: Mnemonic) {
		self.init(mnemonic: mnemonic, passphrase: "")
	}

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
			derivationPaths: paths.map(\.asGeneral)
		)
	}

	public func derivePublicKeys(
		paths: some Collection<some DerivationPathProtocol>,
		factorSourceId: FactorSourceIDFromHash
	) -> [HierarchicalDeterministicFactorInstance] {
		derivePublicKeys(paths: paths).map {
			.init(factorSourceId: factorSourceId, publicKey: $0)
		}
	}

	public func sign(
		hash: Hash,
		path: some DerivationPathProtocol
	) -> SignatureWithPublicKey {
		mnemonicWithPassphraseSign(
			mnemonicWithPassphrase: self,
			derivationPath: path.asGeneral,
			hashToSign: hash
		)
	}
}
