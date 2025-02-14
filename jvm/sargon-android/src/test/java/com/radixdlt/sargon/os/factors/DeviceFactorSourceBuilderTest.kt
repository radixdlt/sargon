package com.radixdlt.sargon.os.factors

import com.radixdlt.sargon.DeviceFactorSourceBuilder
import com.radixdlt.sargon.MnemonicWithPassphrase
import com.radixdlt.sargon.samples.sample
import kotlinx.coroutines.test.StandardTestDispatcher
import kotlinx.coroutines.test.runTest
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class DeviceFactorSourceBuilderTest {

    private val testDispatcher = StandardTestDispatcher()

    @Test
    fun test() = runTest(testDispatcher) {
        var builder = DeviceFactorSourceBuilder()
        val words = MnemonicWithPassphrase.sample().mnemonic.words

        builder = builder.createMnemonicWithPassphraseFromWords(words)
        assertEquals(builder.getMnemonicWithPassphrase().mnemonic.words, words)
    }
}