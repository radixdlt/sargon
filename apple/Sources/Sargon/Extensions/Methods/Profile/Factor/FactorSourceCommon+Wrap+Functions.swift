import Foundation
import SargonUniFFI

extension FactorSourceCommon {
	/// Creates a new `FactorSourceCommon` with crypto parameters
	/// for "Babylon"
	public static func babylon() -> Self {
		newFactorSourceCommonBdfs()
	}

	/// Creates a new `FactorSourceCommon` with crypto parameters
	/// for "Olympia"
	public static func olympia() -> Self {
		newFactorSourceCommonOlympia()
	}
}
