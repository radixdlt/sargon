//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-06-01.
//

import Foundation
import Sargon
import SwiftUI

extension FactorSource {
	public func hintView(
        action: (() -> Void)? = nil
	) -> some SwiftUI.View {
		Group {
			if let device = asDevice {
				device.hint.display()
			} else if let ledger = asLedger {
				ledger.hint.display()
			} else if let arculus = asArculus {
				arculus.hint.display()
			} else if let offDevice = asOffDeviceMnemonic {
				offDevice.hint.display(action: action)
			} else if let securityQuestions = asSecurityQuestions {
				securityQuestions.sealedMnemonic.display(action: action)
			} else if let trustedContact = asTrustedContact {
				trustedContact.contact.display()
			} else {
				Text("No hint")
			}
		}
	}
}

extension TrustedContactFactorSourceContact {
	public func display(
		action: @escaping @Sendable () -> Void = {}
	) -> some SwiftUI.View {
		VStack(alignment: .leading) {
			Labeled("Contact Name", name)
			Labeled("Contact Email", emailAddress)
		}
		.multilineTextAlignment(.leading)
		.frame(maxWidth: .infinity)
	}
}
extension SecurityQuestionsSealedNotProductionReadyMnemonic {
	public func display(
		action: (() -> Void)?
	) -> some SwiftUI.View {
		VStack(alignment: .leading) {
			Labeled("#Questions", self.securityQuestions.count)
            if let action {
                Button("Decrypt with answers", action: action)
            }
		}
		.multilineTextAlignment(.leading)
		.frame(maxWidth: .infinity)
	}
}
extension OffDeviceFactorSourceHint {
	public func display(
		action: (() -> Void)? = nil
	) -> some SwiftUI.View {
		VStack(alignment: .leading) {
			Labeled("Label", displayName)
			if let action {
				Button("Edit Label", action: action)
			}
		}
		.multilineTextAlignment(.leading)
		.frame(maxWidth: .infinity)
	}
}
extension ArculusCardHint {
	public func display() -> some SwiftUI.View {
		VStack(alignment: .leading) {
			Labeled("Arculus Name", name)
			Labeled("Arculus Model", String(describing: model))
		}
		.multilineTextAlignment(.leading)
		.frame(maxWidth: .infinity)
	}
}
extension DeviceFactorSourceHint {
	public func display() -> some SwiftUI.View {
		VStack(alignment: .leading) {
			Labeled("Device Name", name)
			Labeled("Device Model", model)
			Labeled("#Mnemonic Words", mnemonicWordCount.rawValue)
			if let systemVersion {
				Labeled("iOS", systemVersion)
			}
			if let hostAppVersion {
				Labeled("App Version", hostAppVersion)
			}
		}
		.multilineTextAlignment(.leading)
		.frame(maxWidth: .infinity)
	}
}

extension LedgerHardwareWalletHint {
	public func display() -> some SwiftUI.View {
		VStack(alignment: .leading) {
			Labeled("Ledger Name", name)
			Labeled("Ledger Model", model)
		}
		.multilineTextAlignment(.leading)
		.frame(maxWidth: .infinity)
	}
}
