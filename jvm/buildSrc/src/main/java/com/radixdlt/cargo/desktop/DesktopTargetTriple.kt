package com.radixdlt.cargo.desktop

sealed interface DesktopTargetTriple {

    val jnaName: String
    val binaryName: String
    val rustTargetTripleName: String
    object DarwinAArch64: DesktopTargetTriple {
        override val jnaName: String = "darwin-aarch64"
        override val binaryName: String = "libsargon.dylib"
        override val rustTargetTripleName: String = "aarch64-apple-darwin"
    }

    object DarwinX8664: DesktopTargetTriple {
        override val jnaName: String = "darwin-x86-64"
        override val binaryName: String = "libsargon.dylib"
        override val rustTargetTripleName: String = "x86_64-apple-darwin"
    }
    object LinuxArmel: DesktopTargetTriple {
        override val jnaName: String = "linux-armel"
        override val binaryName: String = "libsargon.so"
        override val rustTargetTripleName: String = "aarch64-unknown-linux-gnu"
    }
    object LinuxX8664: DesktopTargetTriple {
        override val jnaName: String = "linux-x86-64"
        override val binaryName: String = "libsargon.so"
        override val rustTargetTripleName: String = "x86_64-unknown-linux-gnu"
    }
    object LinuxWin32X8664: DesktopTargetTriple {
        override val jnaName: String = "win32-x86-64"
        override val binaryName: String = "sargon.dll"
        override val rustTargetTripleName: String = "x86_64-pc-windows-gnu"
    }

    companion object {
        private val all = listOf(
            DarwinAArch64,
            DarwinX8664,
            LinuxArmel,
            LinuxX8664,
            LinuxWin32X8664
        )

        fun current(host: String) : DesktopTargetTriple = all.find {
            it.rustTargetTripleName == host
        } ?: throw RuntimeException("No compatible DesktopTargetTriple found called '$host'")
    }
}