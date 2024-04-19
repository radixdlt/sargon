//
//  File.swift
//
//
//  Created by Alexander Cyon on 2024-04-15.
//

import Foundation
import SargonUniFFI

#if DEBUG
extension AuthorizedDapps {
	public static let sample: Self = newAuthorizedDappsSample()
	public static let sampleOther: Self = newAuthorizedDappsSampleOther()
}
#endif // DEBUG
