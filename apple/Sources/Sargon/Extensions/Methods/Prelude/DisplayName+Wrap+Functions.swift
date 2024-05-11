import Foundation
import SargonUniFFI

extension DisplayName {
	public init(validating name: String) throws {
		self = try newDisplayName(name: name)
	}

    public init(jsonStringLiteral: String) throws {
        self = try newDisplayNameFromJsonString(jsonString: jsonStringLiteral)
    }
    
    public func jsonStringLiteral() -> String {
		displayNameToJsonString(displayName: self)
    }
}
