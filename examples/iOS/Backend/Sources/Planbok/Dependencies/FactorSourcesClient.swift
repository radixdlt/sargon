import DependenciesMacros
import Foundation
import Sargon

// MARK: - FactorSourcesClient
@DependencyClient
public struct FactorSourcesClient: Sendable {
	public typealias AddFactorSource = @Sendable (FactorSource) async throws -> Void
	public typealias UpdateFactorSource = @Sendable (FactorSource) async throws -> Void
	public typealias CreateHWFactorSource = @Sendable (MnemonicWithPassphrase, FactorSourceKind) async throws -> FactorSource
	public typealias CreateSecurityQuestionsFactor = @Sendable (AnswersToQuestions) throws -> SecurityQuestionsNotProductionReadyFactorSource
	public typealias DecryptSecurityQuestionsFactor = @Sendable (AnswersToQuestions, SecurityQuestionsNotProductionReadyFactorSource) throws -> Mnemonic
	public typealias AddAllSampleFactors = @Sendable () async throws -> Void
	public var createHWFactorSource: CreateHWFactorSource
	public var createSecurityQuestionsFactor: CreateSecurityQuestionsFactor
	public var decryptSecurityQuestionsFactor: DecryptSecurityQuestionsFactor
	public var addFactorSource: AddFactorSource
	public var addAllSampleFactors: AddAllSampleFactors
	public var updateFactorSource: UpdateFactorSource
}

// MARK: DependencyKey
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
					LedgerHardwareWalletFactorSource(
						mnemonicWithPassphrase: mnemonicWithPassphrase,
						hint: LedgerHardwareWalletHint(
							label: "Unknown",
							model: .nanoSPlus
						),
						common: FactorSourceCommon.babylon()
					)
					.asGeneral
				case .offDeviceMnemonic:
					OffDeviceMnemonicFactorSource(
						mnemonicWithPassphrase: mnemonicWithPassphrase,
						hint: .init(
							label: "Unknown"
						)
					).asGeneral
				case .arculusCard:
					ArculusCardFactorSource(
						mnemonicWithPassphrase: mnemonicWithPassphrase,
						label: "Unknown"
					).asGeneral
				case .securityQuestions:
					fatalError(
						"SecurityQuestions FS not supported here."
					)
				case .trustedContact:
					fatalError(
						"Trusted Contact not supported yet"
					)
				case .password:
					fatalError(
						"Password not supported yet"
					)
				}

			},
			createSecurityQuestionsFactor: { questionsAndAnswers in
				@Dependency(MnemonicClient.self) var mnemonicClient

				let mnemonic = mnemonicClient.generateNewMnemonic(.twentyFour)
				log.notice("Creating new SecurityQuestions FactorSource, mnemonic is:\n'\(mnemonic.phrase)'\nAnswers:\n\(questionsAndAnswers.map(\.answer))")
				return try SecurityQuestionsNotProductionReadyFactorSource(
					mnemonic: mnemonic,
					questionsAndAnswers: questionsAndAnswers.elements
				)
			},
			decryptSecurityQuestionsFactor: { questionsAndAnswers, factor in
				try factor.decrypt(questionsAndAnswers: questionsAndAnswers.elements)
			},
			addFactorSource: { factorSource in
				log.notice("Adding New factorSource: \(factorSource)")
				_ = try await os.addFactorSource(factorSource: factorSource)
				log.info("Finished adding new factorSource.")
			},
			addAllSampleFactors: {
				log.notice("Adding Many Sample factorSources")
				_ = try await os.debugAddAllSampleFactors()
				log.notice("Finished adding Many Sample factorSources")
			},
			updateFactorSource: { factorSource in
				log.notice("Updating factorSource: \(factorSource)")
				_ = try await os.updateFactorSource(updated: factorSource)
				log.info("Finished updating factorSource.")
			}
		)
	}
}
