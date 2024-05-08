import Foundation
import SargonUniFFI

extension DisplayName {
	public init(validating name: String) throws {
		self = try newDisplayName(name: name)
	}
    
    public init(jsonData: some DataProtocol) throws {
        self = try newDisplayNameFromJsonBytes(jsonBytes: Data(jsonData))
    }
    
    public func jsonData() -> Data {
        displayNameToJsonBytes(displayName: self)
    }
}
