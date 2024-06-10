//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-06-07.
//

import Foundation
import SwiftUI
import Sargon

public struct FactorView: SwiftUI.View {

	public let factor: Factor
	public let pickAction: (() -> Void)?
	public let removeAction: (() -> Void)?
	
	public init(
		factor: Factor,
		pickAction: (() -> Void)? = nil,
		removeAction: (() -> Void)? = nil
	) {
		self.factor = factor
		self.pickAction = pickAction
		self.removeAction = removeAction
	}
	
	public init(
		_ factorSource: FactorSource
	) {
		self.init(factor: Factor(factorSource: factorSource))
	}
	
	public var body: some SwiftUI.View {
		HStack {
			if let pickAction {
				Button(action: pickAction, label: {
					label
				})
			} else {
				label
			}
			
			
			Spacer()
			
			if let removeAction {
				Button(action: removeAction, label: {
					Image(systemName: "plus").rotationEffect(.degrees(45))
				})
			}
		}
	}
	
	@ViewBuilder
	private var label: some View {
		HStack(alignment: .top) {
			if let factorSource = factor.factorSource {
				
				Image(systemName: factorSource.kind.image)
					.resizable()
					.imageScale(.large)
					.aspectRatio(contentMode: .fit)
					.frame(width: 40)
					.rotationEffect(.degrees(factorSource.kind.rotationDegrees))
					.offset(y: factorSource.kind.rotationDegrees > 0 ? 10 : 0)
				
				VStack(alignment: .leading) {
					Text("\(factorSource.kind.title)")
					if let subtitle = factorSource.kind.subtitle {
						Text("\(subtitle)")
							.foregroundStyle(Color.app.gray2)
					}
					Spacer()
				}
			} else {
				Text("Select a factor")
					.fontWeight(.bold)
			}
		}
		.frame(maxWidth: .infinity, idealHeight: 40, alignment: .leading)
		.padding()
		.background(Color.app.white)
		.clipShape(.rect(cornerRadius: 10))
	}
}

extension FactorSourceKind {
	public var image: String {
		switch self {
		case .device: return "lock.iphone"
		case .ledgerHqHardwareWallet: return "mediastick"
		case .arculusCard: return "key.radiowaves.forward"
		case .trustedContact: return "figure.child.and.lock"
		case .securityQuestions: return "person.crop.circle.badge.questionmark"
		case .offDeviceMnemonic: return "key"
		}
	}
	public var rotationDegrees: CGFloat {
		switch self {
		case .ledgerHqHardwareWallet: return 90
		default: return 0
		}
	}
	
}

#Preview {
	ScrollView {
		VStack {
			ForEach(factorSourcesAllSampleValues().shuffled()) {
				FactorView($0)
			}
		}
	}
}
