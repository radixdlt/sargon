//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-15.
//

import Foundation
import SargonUniFFI

public typealias P2PLink = P2pLink

extension P2PLink: SargonModel {}
extension P2PLink: SargonObjectCodable {}

extension P2PLink: Identifiable {
	public typealias ID = PublicKeyHash
}
