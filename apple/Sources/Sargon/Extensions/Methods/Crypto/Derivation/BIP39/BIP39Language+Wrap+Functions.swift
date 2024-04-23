//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-22.
//

import Foundation
import SargonUniFFI

extension BIP39Language {
	public func wordlist() -> [BIP39Word] {
		bip39LanguageWordlist(language: self)
	}
}
