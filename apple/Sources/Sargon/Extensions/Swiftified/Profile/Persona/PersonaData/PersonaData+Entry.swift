//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-21.
//

import Foundation


// MARK: - PersonaData.Entry
extension PersonaData {
	public enum Entry: Sendable, Hashable, Codable, BasePersonaDataEntryProtocol, CustomStringConvertible {
		
		
		
		case name(PersonaDataEntryName)
		case emailAddress(PersonaDataEntryEmailAddress)
		case phoneNumber(PersonaDataEntryPhoneNumber)
	}
}


extension PersonaData.Entry {
	
	public func embed() -> PersonaData.Entry {
		self
	}
	
	public var description: String {
		switch self {
		case let .name(name):
			name.description
		case let .emailAddress(emailAddress):
			emailAddress.email.description
		case let .phoneNumber(phoneNumber):
			phoneNumber.number.description
		}
	}
}

extension PersonaData.Entry {
	public enum Kind: String, Sendable, Hashable, Codable {
		case fullName
		case emailAddress
		case phoneNumber
	}
}
