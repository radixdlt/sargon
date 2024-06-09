//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-06-07.
//

import Foundation
import SwiftUI
import Sargon
import ComposableArchitecture

@Reducer
public struct FactorsBuilderFeature {
	
	@ObservableState
	public struct State: Equatable {
		public enum Mode: Sendable, Hashable {
			case threshold, override
		}
		
		@Shared(.newShieldDraft) var __newShieldDraft
		
		public let mode: Mode
		public let role: Role
		
		public init(mode: Mode, role: Role) {
			self.mode = mode
			self.role = role
		}
		
		public var threshold: FactorThreshold {
			get {
				matrixOfFactorsForRole.threshold
			}
			set {
				matrixOfFactorsForRole.threshold = newValue
			}
		}

		
		public var matrixOfFactorsForRole: MatrixOfFactorsForRole {
			get { __newShieldDraft[role] }
			set {
				__newShieldDraft[role] = newValue
			}
		}
		
		public var factors: Factors {
			get {
				switch mode {
				case .override:
					matrixOfFactorsForRole.overrideFactors
				case .threshold:
					matrixOfFactorsForRole.thresholdFactors
				}
			}
			set {
				switch mode {
				case .override:
					matrixOfFactorsForRole.overrideFactors = newValue
				case .threshold:
					matrixOfFactorsForRole.thresholdFactors = newValue
				}			}
		}
		
		public var pickedFactorID: Factor.ID? {
			get {
				__newShieldDraft.pendingFactorID
			}
			set {
				__newShieldDraft.pendingFactorID = newValue
			}
		}
		
		public var title: LocalizedStringKey {
			switch mode {
			case .override:
				"Override Factors"
			case .threshold:
				"Threshold Factors"
			}
		}
		
		public var canChangeThreshold: Bool {
			mode == .threshold
		}
	}
	
	@CasePathable
	public enum Action: ViewAction {
		
		@CasePathable
		public enum ViewAction {
			case titleButtonTapped
			case appendFactorButtonTapped
			case pickFactorButtonTapped(Factor)
			case removeButtonTapped(Factor)
			case changeThresholdButtonTapped
			case factorsChanged(Factors)
		}
		
		public enum DelegateAction {
			case pickFactor(role: Role)
			case setThreshold(role: Role)
		}
		
		case view(ViewAction)
		case delegate(DelegateAction)
	}
	
	public var body: some ReducerOf<Self> {
		Reduce { state, action in
			switch action {
		
			case .view(.titleButtonTapped):
				log.info("Title button tapped, show important info!")
				return .none
				
			case .view(.appendFactorButtonTapped):
				state.factors.append(Factor())
				return .none
		
			case let .view(.pickFactorButtonTapped(factor)):
				state.pickedFactorID = factor.id
				return .send(.delegate(.pickFactor(role: state.role)))
				
			case let .view(.removeButtonTapped(toRemove)):
				state.__newShieldDraft.removeFactor(toRemove, role: state.role)
				return .none
				
			case .view(.changeThresholdButtonTapped):
				assert(state.mode == .threshold)
				return .send(.delegate(.setThreshold(
					role: state.role
				)))
				
			case let .view(.factorsChanged(new)):
				state.factors = new
				return .none
				
			case .delegate:
				return .none
			}
		}
	}
}
extension FactorsBuilderFeature {
	public typealias HostingFeature = Self
	
	@ViewAction(for: HostingFeature.self)
	public struct View: SwiftUI.View {
		
		@Bindable public var store: StoreOf<HostingFeature>
		
		public init(store: StoreOf<HostingFeature>) {
			self.store = store
		}
		
		public var body: some SwiftUI.View {
			VStack(spacing: 0) {
				HStack {
					Button(
						action: {
							send(.titleButtonTapped)
						},
						label: {
							Label(store.title, systemImage: "info.circle")
								.labelStyle(.flipped())
						}
					)
					Spacer()
				}
				.padding()
				
				Divider().background(Color.app.gray2)
				
				VStack(spacing: 0) {
					ForEach(store.factors) { factor in
						FactorView(
							factor: factor,
							pickAction: {
							
								send(.pickFactorButtonTapped(factor))
							}
						) {
							send(.removeButtonTapped(factor))
						}
					}
					.padding(.horizontal)
					.padding(.top, 10)
					
					Spacer()
					
					Button("Add factors") {
						send(.appendFactorButtonTapped)
					}
					.foregroundStyle(Color.app.gray1.opacity(0.7))
					.padding()
				}
				.frame(maxWidth: .infinity, minHeight: 50)
				.background(Color.app.gray5)
				
				
				Divider().background(Color.app.gray3)
				
				Button(action: {
					send(.changeThresholdButtonTapped)
				}, label: {
					HStack {
						Text("Factors required to sign transactions?")
						Spacer()
						Text("\(store.threshold)")
							.fontWeight(.bold)
							.foregroundStyle(store.canChangeThreshold ? Color.app.blue2 : Color.app.gray2)
					}
					.multilineTextAlignment(.leading)
				})
				.padding()
				.disabled(!store.canChangeThreshold)
				
			}
			.foregroundStyle(Color.app.gray1)
			.overlay(
				RoundedRectangle(cornerRadius: 15)
					.inset(by: 1)
					.stroke(.gray, lineWidth: 1)
			)
			.padding()
			.buttonStyle(.plain)
		}
	}
}

//#Preview {
//	VStack {
//		
//		FactorsBuilderView(
//            factors: .init(get: { [FactorSource.sample].map({ Factor(factorSource: $0) }).asIdentified() }, set: {
//				print("Preview NOOP set factors sources to: \($0)")
//			}),
//			factorThreshold: .threshold(3),
//			title: "Threshold",
//			titleAction: {
//				print("Preview NOOP - titleAction")
//			},
//			changeThresholdAction: { print("Preview NOOP - changeThresholdAction") },
//			pickAction: {
//				print("Preview NOOP - pickAction")
//			}
//		)
//		FactorsBuilderView(
//			factors: .init(get: { [] }, set: {
//				print("Preview NOOP set factors sources to: \($0)")
//			}),
//			factorThreshold: .any,
//			title: "Override",
//			titleAction: {
//				print("Preview NOOP - titleAction")
//			},
//			changeThresholdAction: nil,
//			pickAction: {
//				print("Preview NOOP - pickAction")
//			}
//		)
//	}
//}
// 
