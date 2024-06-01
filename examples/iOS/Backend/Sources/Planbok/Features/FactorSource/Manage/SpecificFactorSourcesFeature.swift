import Foundation
import Sargon
import ComposableArchitecture
import SwiftUI

extension FactorSource {
	public var asDevice: DeviceFactorSource? {
		extract()
	}
	public var asLedger: LedgerHardwareWalletFactorSource? {
		extract()
	}
	public var asArculus: ArculusCardFactorSource? {
		extract()
	}
	public var asOffDeviceMnemonic: OffDeviceMnemonicFactorSource? {
		extract()
	}
	public var asSecurityQuestions: SecurityQuestionsNotProductionReadyFactorSource? {
		extract()
	}
	
	public func hintView() -> some SwiftUI.View {
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
				securityQuestions.sealedMnemonic.display()
			} else {
				Text("No hint")
			}
		}
	}
}
extension SecurityQuestionsSealedNotProductionReadyMnemonic {
	public func display() -> some SwiftUI.View {
		VStack(alignment: .leading) {
			Labeled("#Questions", self.securityQuestions.count)
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


@Reducer
public struct SpecificFactorSourcesFeature {
	
	@ObservableState
	public struct State {
		@SharedReader(.factorSources) var factorSources
		public let kind: FactorSourceKind
	}
	
	@CasePathable
	public enum Action: ViewAction {
		
		@CasePathable
		public enum ViewAction {
			case addNewButtonTapped
		}
		
		case view(ViewAction)
		
		@CasePathable
		public enum DelegateAction {
			case addNew(FactorSourceKind)
		}
		
		case delegate(DelegateAction)
	}
	
	public init() {}
	
	public var body: some ReducerOf<Self> {
		Reduce { state, action in
			switch action {
			case .view(.addNewButtonTapped):
				return .send(.delegate(.addNew(state.kind)))
		
			default:
				return .none
				
			}
		}
	}
}

extension SpecificFactorSourcesFeature {
	public typealias HostingFeature = Self
	
	@ViewAction(for: HostingFeature.self)
	public struct View: SwiftUI.View {
		
		@Bindable public var store: StoreOf<HostingFeature>
		
		public var kind: FactorSourceKind {
			store.state.kind
		}
		public var factors: IdentifiedArrayOf<FactorSource> {
			store.state.factorSources.filter(kind: kind)
		}
		
		public var body: some SwiftUI.View {
			VStack {
				Text("\(kind) Factors").font(.largeTitle)
		
				if factors.isEmpty {
					Text("You have no factors")
				} else {
					ScrollView {
						ForEach(factors, id: \.id) { factorSource in
							VStack {
								FactorSourceCardView(factorSource: factorSource)
							}
						}
					}
				}
				
				Spacer()
		   
				Button("Add New") {
					send(.addNewButtonTapped)
				}
			}
			.padding(.bottom, 100)
		}
	}
	
}

