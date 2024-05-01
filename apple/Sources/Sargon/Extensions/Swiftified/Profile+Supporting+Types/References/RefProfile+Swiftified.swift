//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-05-01.
//

import Foundation
import SargonUniFFI

public protocol SargonReferenceType: SargonModel {
	associatedtype Inner: SargonModel
	func take() throws -> Inner
	static func from(inner: Inner) -> Self
}

extension RefProfile: SargonReferenceType {
	public typealias Inner = Profile
	
	public static func from(inner: Inner) -> Self {
		RefProfile(inner: inner) as! Self
	}
}
