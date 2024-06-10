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
		Group {
			if let factorSource = factor.factorSource {
				HStack {
					if let factorImageName = factorSource.kind.image {
						Image(systemName: factorImageName)
							.imageScale(.large)
						
					}
					VStack(alignment: .leading) {
						Text("\(factorSource.kind.title)")
						if let subtitle = factorSource.kind.subtitle {
							Text("\(subtitle)")
								.foregroundStyle(Color.app.gray2)
						}
					}
				}
			} else {
				Text("Select a factor")
					.fontWeight(.bold)
			}
		}
		.frame(maxWidth: .infinity, alignment: .leading)
		.padding()
		.background(Color.app.white)
		.clipShape(.rect(cornerRadius: 10))
	}
}
