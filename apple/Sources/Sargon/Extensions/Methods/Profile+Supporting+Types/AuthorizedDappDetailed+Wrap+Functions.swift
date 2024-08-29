//
//  File.swift
//  
//
//  Created by Matias Bzurovski on 29/8/24.
//

import Foundation
import SargonUniFFI

extension AuthorizedDappDetailed {
	public mutating func showDeposits(_ show: Bool) {
		preferences.deposits = show ? .visible : .hidden
	}
	
	public var isDepositsVisible: Bool {
		preferences.deposits == .visible
	}
}
