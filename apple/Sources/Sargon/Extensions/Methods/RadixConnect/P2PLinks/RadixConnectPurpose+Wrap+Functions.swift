import Foundation
import SargonUniFFI

extension RadixConnectPurpose {
    
    public init(string: String) {
        self = newRadixConnectPurposeFromString(string: string)
    }

    public init(jsonData: some DataProtocol) throws {
		self = try newRadixConnectPurposeFromJsonBytes(jsonBytes: Data(jsonData))
	}
	
	public func jsonData() -> Data {
        radixConnectPurposeToJsonBytes(radixConnectPurpose: self)
	}
}
