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
			case deviceFactorSourcesButtonTapped
		}
		
		case view(ViewAction)
		
		@CasePathable
		public enum DelegateAction {
			case navigate(Navigate)
			
			@CasePathable
			public enum Navigate {
				case toDeviceFactorSources
			}
		}
		
		case delegate(DelegateAction)
	}
	
	public init() {}
	
	public var body: some ReducerOf<Self> {
		Reduce { state, action in
			switch action {
			case .view(.deviceFactorSourcesButtonTapped):
				return .send(.delegate(.navigate(.toDeviceFactorSources)))
				
		
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
		   
				
			}
			.padding(.bottom, 100)
		}
	}
	
}

public struct FactorSourceCardView<F: FactorSourceProtocol>: SwiftUI.View {
	public let factorSource: F
	public var body: some SwiftUI.View {
		VStack(alignment: .leading) {
			Text("Kind: \(factorSource.kind)")
			Text("Added: \(factorSource.addedOn.formatted(.dateTime))")
			Text("Last Used: \(factorSource.lastUsedOn.formatted(.dateTime))")
		}
		.multilineTextAlignment(.leading)
		.frame(maxWidth: .infinity)
		.background(Color.orange)
		.padding()
	}
}
