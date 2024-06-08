//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-06-07.
//

import Foundation
import Sargon
import ComposableArchitecture
import SwiftUI

@Reducer
public struct PickFactorSourceFeature {
    
    @ObservableState
    public struct State: Equatable {
		
        @SharedReader(.factorSources) var factorSources
		@Shared(.newShieldDraft) var __newShieldDraft
		public var pickedFactor: Factor? {
			get {
				__newShieldDraft.pendingFactor
			}
			set {
				__newShieldDraft.pendingFactor = newValue
			}
		}
		public func isFactorSourceAvailable(id: FactorSourceID) -> Bool {
			__newShieldDraft.usedFactorSources.contains(where: { $0.id == id }) == false
		}
        public var idOfSelected: FactorSourceID? = nil
		public let role: Role
        public let kind: FactorSourceKind
    }
    
    @CasePathable
    public enum Action: ViewAction {
        @CasePathable
        public enum ViewAction {
            case tappedFactorSource(id: FactorSourceID)
            case confirmSelectedFactorButtonTapped
        }
        public enum DelegateAction {
            case done
        }
        case view(ViewAction)
        case delegate(DelegateAction)
    }
    
    public var body: some ReducerOf<Self> {
        Reduce { state, action in
            switch action {
                
            case .view(.confirmSelectedFactorButtonTapped):
                guard
                    let selected = state.idOfSelected,
                    let factor = state.factorSources[id: selected]
                else {
                    return .none
                }
                state.pickedFactor?.factorSource = factor
                return .send(.delegate(.done))
                
            case let .view(.tappedFactorSource(id)):
                if let selected = state.idOfSelected,
                   id == selected {
                    state.idOfSelected = nil
                } else {
                    state.idOfSelected = id
                }
                return .none
                
            case .delegate:
                return .none

            }
        }
    }
}

extension FactorSource {
    public var displayLabel: String {
        switch self {
        case let .device(value): "\(value.hint.name) (\(value.hint.model))"
        case let .ledger(value):"\(value.hint.name) (\(value.hint.model))"
        case let .arculusCard(value): "\(value.hint.name) (\(value.hint.model))"
        case let .offDeviceMnemonic(value): "\(value.hint.displayName)"
        case let .trustedContact(value): "\(value.contact.name) (\(value.contact.emailAddress.email))"
        case let .securityQuestions(value): "Questions: \(value.sealedMnemonic.securityQuestions.map({ q in q.id.description }).joined(separator: ", "))"
        }
    }
}

extension PickFactorSourceFeature {
    public typealias HostingFeature = Self
    
    @ViewAction(for: HostingFeature.self)
    public struct View: SwiftUI.View {
        
        public let store: StoreOf<HostingFeature>
        
        public var kind: FactorSourceKind {
            store.kind
        }
        
        public var factors: IdentifiedArrayOf<FactorSource> {
            store.factorSources.filter(kind: kind)
        }
        
        public init(store: StoreOf<HostingFeature>) {
            self.store = store
        }
        
        public struct SelectableFactorView: SwiftUI.View {
           
			public let factorSource: FactorSource
            public let isAvailable: Bool
            public let isSelected: Bool
            
			public let action: () -> Void
			
            public var body: some SwiftUI.View {
                Button(action: action, label: {
                    HStack {
                        VStack(alignment: .leading) {
                            Text("\(factorSource.displayLabel)")
                            Labeled("Last Used", factorSource.lastUsedOn.formatted(.dateTime))
                            Labeled("Added", factorSource.addedOn.formatted(.dateTime))
                            Labeled("ID", "...\(factorSource.id.description.suffix(6))").font(.footnote)
							
							if !isAvailable {
								Text("ALREADY USED IN SHIELD").fontWeight(.bold)
							}
                        }
                        .multilineTextAlignment(.leading)

						if isAvailable {
							Circle()
								.stroke(Color.app.gray2, lineWidth: 3)
								.fill(isSelected ? Color.app.gray2 : Color.app.gray5)
								.frame(width: 20, height: 20)
						}
                    }
                    .padding()
                    .background(Color.app.white)
                    .foregroundStyle(Color.app.gray1)
                    .overlay(
                        RoundedRectangle(cornerRadius: 15)
                            .inset(by: 1)
                            .stroke(.gray, lineWidth: 1)
                    )
                    .padding()
                })
				.disabled(!isAvailable)
                .buttonStyle(.plain)
         
            }
        }
        
        public var body: some SwiftUI.View {
            VStack {
                Text("Select A Factor")
                    .font(.largeTitle)

				Text("For \(store.role)")
				
                Text("You have the the following \(store.kind) factors")
                
                ScrollView {
                    VStack {
                        ForEach(factors) { factorSource in
                            SelectableFactorView(
                                factorSource: factorSource,
								isAvailable: store.state.isFactorSourceAvailable(id: factorSource.id),
                                isSelected: factorSource.id == store.idOfSelected
                            ) {
                                send(.tappedFactorSource(id: factorSource.id))
                            }
                        }
                    }
                 
                }

                Button("Confirm") {
                    send(.confirmSelectedFactorButtonTapped)
                }
                .buttonStyle(.borderedProminent)
                .disabled(store.idOfSelected == nil)
            }
            .background(Color.app.gray5)
            .padding()
        }
    }
}
