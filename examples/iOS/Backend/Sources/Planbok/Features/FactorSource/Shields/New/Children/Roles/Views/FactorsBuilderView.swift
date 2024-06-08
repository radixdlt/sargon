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

public struct FactorsBuilderView: SwiftUI.View {
	@Shared(.newShieldDraft) var __newShieldDraft
	public var pickedFactor: Factor? {
		get {
			__newShieldDraft.pendingFactor
		}
	}
	@Binding var factors: IdentifiedArrayOf<Factor>
	
	public let factorThreshold: FactorThreshold
	
	public let title: LocalizedStringKey
	public let titleAction: () -> Void
	public let changeThresholdAction: (() -> Void)?
	public let pickAction: () -> Void
	
	public var body: some SwiftUI.View {
		VStack(spacing: 0) {
			HStack {
				Button(
					action: titleAction,
					label: {
						Label(title, systemImage: "info.circle")
							.labelStyle(.flipped())
					}
				)
				Spacer()
			}
			.padding()
			
			Divider().background(Color.app.gray2)
			
			VStack(spacing: 0) {
				ForEach(factors) { factor in
					FactorView(
						factor: factor,
                        pickAction: {
							__newShieldDraft.pendingFactor = factor
                            pickAction()
                        }
					) {
						self.factors.remove(
							id: factor.id
						)
					}
				}
				.padding(.horizontal)
				.padding(.top, 10)
				
				Spacer()
				
				Button("Add factors") {
					self.factors.append(Factor(factorSource: nil))
				}
                .foregroundStyle(Color.app.gray1.opacity(0.7))
				.padding()
			}
			.frame(maxWidth: .infinity, minHeight: 50)
			.background(Color.app.gray5)
			
			
			Divider().background(Color.app.gray3)
			
			Button(action: {
				changeThresholdAction?()
			}, label: {
				HStack {
					Text("Factors required to sign transactions?")
					Spacer()
					Text("\(factorThreshold)")
						.fontWeight(.bold)
						.foregroundStyle(changeThresholdAction == nil ? Color.app.gray2 : Color.app.blue2)
				}
				.multilineTextAlignment(.leading)
			})
			.padding()
			.disabled(changeThresholdAction == nil)
			
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

#Preview {
	VStack {
		
		FactorsBuilderView(
            factors: .init(get: { [FactorSource.sample].map({ Factor(factorSource: $0) }).asIdentified() }, set: {
				print("Preview NOOP set factors sources to: \($0)")
			}),
			factorThreshold: .threshold(3),
			title: "Threshold",
			titleAction: {
				print("Preview NOOP - titleAction")
			},
			changeThresholdAction: { print("Preview NOOP - changeThresholdAction") },
			pickAction: {
				print("Preview NOOP - pickAction")
			}
		)
		FactorsBuilderView(
			factors: .init(get: { [] }, set: {
				print("Preview NOOP set factors sources to: \($0)")
			}),
			factorThreshold: .any,
			title: "Override",
			titleAction: {
				print("Preview NOOP - titleAction")
			},
			changeThresholdAction: nil,
			pickAction: {
				print("Preview NOOP - pickAction")
			}
		)
	}
}
 
