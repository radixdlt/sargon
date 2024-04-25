//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-23.
//

import Foundation
import SargonUniFFI

extension FactorSourceCryptoParameters {
	public static let babylon: Self = newFactorSourceCryptoParametersPresetBabylonOnly()
	
	public static let olympia: Self = newFactorSourceCryptoParametersPresetOlympiaOnly()
	
	public static let babylonOlympiaCompatible: Self = newFactorSourceCryptoParametersPresetBabylonOlympiaCompatible()
	
	public var supportsOlympia: Bool {
		factorSourceCryptoParametersSupportsOlympia(parameters: self)
	}
	
	public var supportsBabylon: Bool {
		factorSourceCryptoParametersSupportsBabylon(parameters: self)
	}
}
