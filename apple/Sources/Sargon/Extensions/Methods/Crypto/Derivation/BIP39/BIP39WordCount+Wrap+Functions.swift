import Foundation
import SargonUniFFI

extension BIP39WordCount: CaseIterable {
	public static var allCases: [Self] {
		bip39WordCountAll()
	}
}
