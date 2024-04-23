//
//  File.swift
//
//
//  Created by Alexander Cyon on 2024-04-19.
//

import Foundation
import SargonUniFFI

#if DEBUG
extension AppPreferences {
	public static let sample: Self = newAppPreferencesSample()
	public static let sampleOther: Self = newAppPreferencesSampleOther()
}
#endif // DEBUG
