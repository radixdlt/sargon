import Foundation
import Sargon
import ComposableArchitecture
import SwiftUI

@Reducer
public struct ManageSpecificFactorSourcesFeature {
	
	@Reducer(state: .equatable)
	public enum Destination {
		case decryptSecurityQuestions(DecryptSecurityQuestionsFeatureCoordinator)
	}
	
	@ObservableState
	public struct State {
		@SharedReader(.factorSources) var factorSources

		@Presents var destination: Destination.State?
		public let kind: FactorSourceKind
	}
	
	@CasePathable
	public enum Action: ViewAction {
		
		@CasePathable
		public enum ViewAction {
			case addNewButtonTapped
			case factorSourceActionButtonTapped(FactorSource)
		}
		
		case view(ViewAction)
		case destination(PresentationAction<Destination.Action>)
		
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
		
			case let .view(.factorSourceActionButtonTapped(factorSource)):
				if let securityQuestions = factorSource.asSecurityQuestions {
					state.destination = .decryptSecurityQuestions(
						DecryptSecurityQuestionsFeatureCoordinator.State(
							securityQuestionsFactorSource: securityQuestions
						)
					)
				} else {
					log.warning("FactorSource tapped but no action performed: \(factorSource)")
				}
				return .none

			case .destination(.presented(.decryptSecurityQuestions(.delegate(.done)))):
				state.destination = nil
				return .none

			default:
				return .none
				
			}
		}
		.ifLet(\.$destination, action: \.destination)
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
								FactorSourceCardView(factorSource: factorSource) {
									send(.factorSourceActionButtonTapped(factorSource))
								}
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
			.sheet(
				item: $store.scope(
					state: \.destination?.decryptSecurityQuestions,
					action: \.destination.decryptSecurityQuestions
				)
			) { store in
				DecryptSecurityQuestionsFeatureCoordinator.View(store: store)
			}
		}
	}
	
}

