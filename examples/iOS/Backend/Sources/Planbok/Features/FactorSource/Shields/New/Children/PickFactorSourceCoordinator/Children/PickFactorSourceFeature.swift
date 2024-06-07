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
        @Shared(.pickedFactor) var pickedFactor
        public var idOfSelected: FactorSourceID? = nil
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
                guard let selected = state.idOfSelected, let factor = state.factorSources[id: selected] else {
                    return .none
                }
                log.notice("PickFactorSourceFeature: Setting pickedFactor to: \(factor)")
                state.pickedFactor?.factorSource = factor
                return .send(.delegate(.done))
           
            case let .view(.tappedFactorSource(id)):
                if let selected = state.idOfSelected,  id == selected {
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
            public let isSelected: Bool
            public let action: () -> Void
            public var body: some SwiftUI.View {
                HStack {
                    Button(action: action, label: {
                        factorSource.hintView()
                    })
                    
                    Text(isSelected ? "✅" : "☑️")
                }
            }
        }
        
        public var body: some SwiftUI.View {
            VStack {
                Text("Select A Factor")
                    .font(.largeTitle)

                Text("You have the the following \(store.kind) factors")
                
                ScrollView {
                    ForEach(factors) { factorSource in
                        let isSelected = factorSource.id == store.idOfSelected
                        SelectableFactorView(
                            factorSource: factorSource,
                            isSelected: isSelected
                        ) {
                            send(.tappedFactorSource(id: factorSource.id))
                        }
                    }
                }

                Button("Confirm") {
                    send(.confirmSelectedFactorButtonTapped)
                }
                .buttonStyle(.borderedProminent)
                .disabled(store.idOfSelected == nil)
            }
        }
    }
}
