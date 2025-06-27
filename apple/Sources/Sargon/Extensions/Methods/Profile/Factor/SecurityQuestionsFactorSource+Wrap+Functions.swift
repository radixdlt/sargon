// import Foundation
// import SargonUniFFI
//
// extension SecurityQuestionsNotProductionReadyFactorSource {
//	public init(
//		mnemonic: Mnemonic,
//		questionsAndAnswers: [SecurityNotProductionReadyQuestionAndAnswer]
//	) throws {
//		self = try newSecurityQuestionsFactorSourceByEncryptingMnemonic(
//			mnemonic: mnemonic,
//			with: questionsAndAnswers
//		)
//	}
//
//	public func decrypt(questionsAndAnswers: [SecurityNotProductionReadyQuestionAndAnswer]) throws -> Mnemonic {
//		try securityQuestionsFactorSourceDecrypt(factorSource: self, with: questionsAndAnswers)
//	}
// }
