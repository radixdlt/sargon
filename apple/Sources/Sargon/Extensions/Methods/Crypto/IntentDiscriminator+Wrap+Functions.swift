import SargonUniFFI

extension IntentDiscriminator {
	public static func secureRandom() -> Self {
		newIntentDiscriminatorRandom()
	}

	public var value: UInt64 {
		intentDiscriminatorGetValue(intentDiscriminator: self)
	}
}
