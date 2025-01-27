import Foundation
import SargonUniFFI

extension DerivationPath {
	public var path: HDPath {
		derivationPathToHdPath(path: self)
	}

	/// Returns a CAP 43 String representation of this `DerivationPath`.
	/// Useful when we need to log or debug, but should **never** be used when communicating with external APIs.
	public func toString() -> String {
		derivationPathToString(path: self)
	}

	/// Returns a BIP 32 String representation of this `DerivationPath`.
	/// Needed to communicate with external APIs such as Arculus or Ledger.
	public func toBip32String() -> String {
		derivationPathToBip32String(path: self)
	}

	/// Attempts to build a DerivationPath from a CAP 43 or BIP 32 representation.
	/// The initializer is lenient, so it will attempt with the latter if the first fails
	public init(string: String) throws {
		self = try newDerivationPathFromString(string: string)
	}
}
