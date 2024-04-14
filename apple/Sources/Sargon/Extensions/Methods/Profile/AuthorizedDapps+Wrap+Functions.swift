//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-14.
//

import Foundation
import SargonUniFFI

extension AuthorizedDapps {
	public init(_ elements: [AuthorizedDapp]) {
		self = newAuthorizedDapps(authorizedDapps: elements)
	}
	
	public var elements: [AuthorizedDapp] {
		getAuthorizedDapps(authorizedDapps: self)
	}
}