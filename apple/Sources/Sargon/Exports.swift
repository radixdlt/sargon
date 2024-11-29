// ADVANTAGE:
// This makes it possible to write `Sargon.AppearanceId` in iOS Wallet App, instead of having to write `SargonUniFFI.AppearanceId`,
// which is kind of important since `SargonUniFFI.AppearanceID` will not compile, since it is a typealias defined by `Sargon`.
// So we would like iOS Wallet App to not have to be aware of where the types are defined, either `SargonUniFFI` or `Sargon`.
//
// DISADVANTAGE:
// Here in Sargon we can refer to symbols inside of `SargonUniFFI` without having to do `import SargonUniFFI` in some file `Foo.swift`
// unfortunately since Xcode 15 the `@_exported import SargonUniFFI` only helps with COMPILATION but NOT code HIGHLIGHTING, meaning that
// we get no code highlighting of those `SargonUniFFI` symbols inside of `Foo.swift`.
@_exported import SargonUniFFI
