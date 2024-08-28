//
//  File.swift
//
//
//  Created by Alexander Cyon on 2024-04-18.
//

import Foundation
import SargonUniFFI

extension AuthorizedDapp {
	
	public init(jsonData: some DataProtocol) throws {
		self = try newAuthorizedDappFromJsonBytes(jsonBytes: Data(jsonData))
	}
	
	public func jsonData() -> Data {
		authorizedDappToJsonBytes(authorizedDapp: self)
	}
	
	public mutating func showDeposits(_ show: Bool) {
		preferences.deposits = show ? .visible : .hidden
	}
	
	public var showDeposits: Bool {
		preferences.deposits == .visible
	}
}
