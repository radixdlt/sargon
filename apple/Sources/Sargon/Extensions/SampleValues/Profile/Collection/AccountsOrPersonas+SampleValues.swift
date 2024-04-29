//
//  File.swift
//
//
//  Created by Alexander Cyon on 2024-04-15.
//

import Foundation
import SargonUniFFI

#if DEBUG
extension AccountsOrPersonas {
	public static let sample: Self = newAccountsOrPersonasSample()
	public static let sampleOther: Self = newAccountsOrPersonasSampleOther()
}
#endif // DEBUG
