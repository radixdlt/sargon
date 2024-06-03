//
//  File.swift
//
//
//  Created by Alexander Cyon on 2024-05-05.
//

import Foundation
import Sargon
import DependenciesMacros

@DependencyClient
public struct FactorSourcesClient: Sendable {
	public typealias AddFactorSource = @Sendable (FactorSource) async throws -> Void
	public typealias CreateHWFactorSource = @Sendable (MnemonicWithPassphrase, FactorSourceKind) async throws -> FactorSource
	public typealias CreateSecurityQuestionsFactor = @Sendable (AnswersToQuestions) async throws -> SecurityQuestionsNotProductionReadyFactorSource
	public var createHWFactorSource: CreateHWFactorSource
	public var createSecurityQuestionsFactor: CreateSecurityQuestionsFactor
	public var addFactorSource: AddFactorSource
}

extension FactorSourcesClient: DependencyKey {
	public static let liveValue = Self.live(os: SargonOS.shared)
	public static func live(os: SargonOS) -> Self {
		Self(
			createHWFactorSource: { mnemonicWithPassphrase, kind -> FactorSource in
				switch kind {
				case .device:
					try await os.createDeviceFactorSource(
						mnemonicWithPassphrase: mnemonicWithPassphrase,
						factorType: mnemonicWithPassphrase.mnemonic.wordCount == .twentyFour ? .babylon(
							isMain: false
						) : .olympia
					).asGeneral
					
				case .ledgerHqHardwareWallet:
					LedgerHardwareWalletFactorSource.init(
						mnemonicWithPassphrase: mnemonicWithPassphrase,
						hint: LedgerHardwareWalletHint(
							name: "Unknown",
							model: .nanoSPlus
						),
						common: FactorSourceCommon.babylon()
					)
					.asGeneral
				case .offDeviceMnemonic:
					OffDeviceMnemonicFactorSource.init(
						mnemonicWithPassphrase: mnemonicWithPassphrase,
						hint: .init(
							displayName: "Unknown"
						)
					).asGeneral
				case .arculusCard:
					ArculusCardFactorSource(
						mnemonicWithPassphrase: mnemonicWithPassphrase,
						name: "Unknown"
					).asGeneral
				case .securityQuestions:
					fatalError(
						"SecurityQuestions FS not supported here."
					)
					
				case .trustedContact:
					fatalError(
						"Trusted Contact not supported yet"
					)
				}
				
			},
			createSecurityQuestionsFactor: { qas in
				let mnemonic = mnemonic
			},
			addFactorSource: { factorSource in
				log.notice("Adding New factorSource: \(factorSource)")
				let _ = try await os.addFactorSource(factorSource: factorSource)
				log.info("Finished adding new factorSource.")
			}
		)
	}
}
