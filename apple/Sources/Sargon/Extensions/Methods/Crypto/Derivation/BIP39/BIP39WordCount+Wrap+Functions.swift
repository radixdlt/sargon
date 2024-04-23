//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-22.
//

import Foundation
import SargonUniFFI

extension BIP39WordCount {
	public static var allCases: [Self] {
		bip39WordCountAll()
	}
}
