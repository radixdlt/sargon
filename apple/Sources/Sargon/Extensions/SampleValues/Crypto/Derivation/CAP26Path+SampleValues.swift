#if DEBUG
extension CAP26Path {
    public static let sample: Self = .account(value: AccountPath.sample)
    public static let sampleOther: Self = .identity(value: IdentityPath.sample)
}
#endif // DEBUG
