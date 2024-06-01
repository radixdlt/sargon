import Foundation
import Sargon
import ComposableArchitecture
import SwiftUI

@Reducer
public struct ManageSpecificFactorSourcesFeature {
	
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

extension ManageSpecificFactorSourcesFeature {
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

