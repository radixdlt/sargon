//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-14.
//

import Foundation
import SargonUniFFI

extension P2pLinks {
	public init(_ elements: [P2pLink]) {
		self = newP2PLinks(p2PLinks: elements)
	}
	
	public var elements: [P2pLink] {
		getP2PLinks(p2PLinks: self)
	}
}
