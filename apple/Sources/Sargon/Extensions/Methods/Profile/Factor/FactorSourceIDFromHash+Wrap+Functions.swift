import Foundation
import SargonUniFFI

extension FactorSourceIDFromHash {
	public init(kind: FactorSourceKind, mnemonicWithPassphrase: MnemonicWithPassphrase) {
		self = newFactorSourceIdFromHashFromMnemonicWithPassphrase(
			factorSourceKind: kind,
			mnemonicWithPassphrase: mnemonicWithPassphrase
		)
	}

	public init(jsonData: some DataProtocol) throws {
		self = try newFactorSourceIDFromHashFromJsonBytes(jsonBytes: Data(jsonData))
	}

	public func jsonData() -> Data {
		factorSourceIDFromHashToJsonBytes(factorSourceIDFromHash: self)
	}

	public func toString() -> String {
		factorSourceIdFromHashToString(factorSourceId: self)
	}

	public func spotCheck(input: SpotCheckInput) -> Bool {
		factorSourceIdFromHashPerformSpotCheck(factorSourceIdFromHash: self, input: input)
	}
}
