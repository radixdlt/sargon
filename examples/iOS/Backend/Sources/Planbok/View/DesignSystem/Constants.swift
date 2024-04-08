//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-02-16.
//

import Foundation

public typealias Grid = CGFloat

extension Grid {
	fileprivate static let unit: Self = 4
}

extension Grid {
	/// 72
	public static let huge1 = unit * 18

	/// 60
	public static let huge2 = unit * 15

	/// 48
	public static let huge3 = unit * 12

	/// 40
	public static let large1 = unit * 10

	/// 32
	public static let large2 = unit * 8

	/// 28
	public static let large3 = unit * 7

	/// 24
	public static let medium1 = unit * 6

	/// 20
	public static let medium2 = unit * 5

	/// 16
	public static let medium3 = unit * 4

	/// 12
	public static let small1 = unit * 3

	/// 8
	public static let small2 = unit * 2

	/// 4
	public static let small3 = unit * 1
}
