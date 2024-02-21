// swift-tools-version:5.9

// This is a HACKY workaround the fact that SPM does not allow for Package level exclusion
// of files/folders. SPM actually HAD support for it but it was removed in 2017, in PR
// https://github.com/apple/swift-package-manager/commit/cb69accf41da55386f9703308958aa49ca2a4c5f
//
// So instead we have to add an empty dummy Package.swift to each folder we wanna hide, as per:
// See: https://github.com/apple/swift-package-manager/issues/4460#issuecomment-1475025748
// And: https://stackoverflow.com/questions/69382302/swift-package-how-to-exclude-files-in-root-git-directory-from-the-actual-swift/70990534#70990534
// And: https://github.com/tuist/tuist/pull/2058
import PackageDescription

let package = Package()