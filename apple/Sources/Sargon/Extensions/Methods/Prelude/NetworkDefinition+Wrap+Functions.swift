import Foundation
import SargonUniFFI

extension NetworkDefinition {
	public static func lookupBy(logicalName: String) throws -> Self {
		try newNetworkDefinitionLookupByName(logicalName: logicalName)
	}
}
