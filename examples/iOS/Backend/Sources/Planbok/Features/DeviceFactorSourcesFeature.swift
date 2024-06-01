import Foundation
import Sargon
import ComposableArchitecture
import SwiftUI

@Reducer
public struct DeviceFactorSourcesFeature {

	@ObservableState
	public struct State: Equatable {
		@SharedReader(.deviceFactorSources) var deviceFactorSources
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
			case navigate(Navigate)
			
			@CasePathable
			public enum Navigate {
				case toNewDeviceFactorSource
			}
		}
		
		case delegate(DelegateAction)
	}
	
	public init() {}
	
	public var body: some ReducerOf<Self> {
		Reduce { state, action in
			switch action {
			case .view(.addNewButtonTapped):
				return .send(.delegate(.navigate(.toNewDeviceFactorSource)))
		
			default:
				return .none
				
			}
		}
	}
}

extension DeviceFactorSourcesFeature {
	
	@ViewAction(for: DeviceFactorSourcesFeature.self)
	public struct View: SwiftUI.View {
		
		@Bindable public var store: StoreOf<DeviceFactorSourcesFeature>
		
		public var body: some SwiftUI.View {
			VStack {
				Text("Device FactorSources").font(.largeTitle)
		
				if store.state.deviceFactorSources.isEmpty {
					Text("You have no DeviceFactorSources")
				} else {
					ScrollView {
						ForEach(store.state.deviceFactorSources) { factorSource in
							VStack {
								FactorSourceCardView(factorSource: factorSource)
							}
						}
					}
				}
		   
				Button("Add New") {
					send(.addNewButtonTapped)
				}
			}
			.padding(.bottom, 100)
		}
	}
	
}

extension DeviceFactorSourceHint: FactorSourceHint {
	public func display() -> any SwiftUI.View {
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
extension DeviceFactorSource: DisplayableFactorSource {}
