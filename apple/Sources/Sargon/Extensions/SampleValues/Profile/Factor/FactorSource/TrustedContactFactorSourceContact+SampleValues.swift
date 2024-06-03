//
//  File.swift
//
//
//  Created by Alexander Cyon on 2024-05-28.
//

import Foundation
import SargonUniFFI

#if DEBUG
extension EmailAddress {
	public static let sample: Self = newEmailAddressSample()
	public static let sampleOther: Self = newEmailAddressSampleOther()
}
#endif // DEBUG

#if DEBUG
extension TrustedContactFactorSourceContact {
	// FIXME replace with Sargon ones
	public static let sample: Self = newTrustedContactFactorSourceContactSample()
	public static let sampleOther: Self = newTrustedContactFactorSourceContactSampleOther()
}

#endif // DEBUG
