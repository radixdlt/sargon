//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-21.
//

import Foundation
import SargonUniFFI

public typealias PersonaDataEntryEmailAddress = EmailAddress
extension EmailAddress: SargonModel {}
extension EmailAddress: SargonStringCodable {}
extension EmailAddress: CustomStringConvertible {
	public var description: String {
		email
	}
}

// MARK: - EmailAddress + PersonaDataEntryProtocol
extension EmailAddress: PersonaDataEntryProtocol {
	public static var kind: PersonaData.Entry.Kind {
		.emailAddress
	}
	
	public static func extract(from entry: PersonaData.Entry) -> Self? {
		guard case let .emailAddress(value) = entry else { return nil }
		return value
	}
	
	public func embed() -> PersonaData.Entry {
		.emailAddress(self)
	}
}
