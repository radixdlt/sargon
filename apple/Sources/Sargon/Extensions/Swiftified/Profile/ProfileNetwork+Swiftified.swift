//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-15.
//

import Foundation
import SargonUniFFI

extension ProfileNetwork: SargonModel {}
extension ProfileNetwork: Identifiable {
	public typealias ID = NetworkID
}
