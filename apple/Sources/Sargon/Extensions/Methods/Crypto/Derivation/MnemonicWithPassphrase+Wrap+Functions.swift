//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-19.
//

import Foundation
import SargonUniFFI

extension MnemonicWithPassphrase {
	public init(jsonData: some DataProtocol) throws {
		self = try newMnemonicWithPassphraseFromJsonBytes(jsonBytes: Data(jsonData))
	}
	
	public func jsonData() -> Data {
		mnemonicWithPassphraseToJsonBytes(mnemonicWithPassphrase: self)
	}
    
    
    public func validate(
        publicKeys: some Collection<HierarchicalDeterministicPublicKey>
    ) -> Bool {
        mnemonicWithPassphraseValidatePublicKeys(mnemonicWithPassphrase: self, hdKeys: Array(publicKeys))
    }
}
