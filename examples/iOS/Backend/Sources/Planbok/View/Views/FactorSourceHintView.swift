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
		action: @escaping @Sendable () -> Void = {}
	) -> some SwiftUI.View {
		Group {
			if let device = asDevice {
				device.hint.display()
			} else if let ledger = asLedger {
				ledger.hint.display()
			} else if let arculus = asArculus {
				arculus.hint.display()
			} else if let offDevice = asOffDeviceMnemonic {
				offDevice.hint.display()
			} else if let securityQuestions = asSecurityQuestions {
				securityQuestions.sealedMnemonic.display(action: action)
			} else {
				Text("No hint")
			}
		}
	}
}

extension SecurityQuestionsSealedNotProductionReadyMnemonic {
	public func display(
		action: @escaping @Sendable () -> Void = {}
	) -> some SwiftUI.View {
		VStack(alignment: .leading) {
			Labeled("#Questions", self.securityQuestions.count)
			Button("Test", action: action)
		}
		.multilineTextAlignment(.leading)
		.frame(maxWidth: .infinity)
	}
}
extension OffDeviceFactorSourceHint {
	public func display() -> some SwiftUI.View {
		VStack(alignment: .leading) {
			Labeled("Label", displayName)
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
