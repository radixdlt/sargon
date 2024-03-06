package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.string
import com.radixdlt.sargon.sample.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertNotEquals
import org.junit.jupiter.api.Test

class TransactionManifestTest {

    @Test
    fun test() {
        assertEquals(TransactionManifest.sample(), TransactionManifest.sample())
        assertEquals(TransactionManifest.sample.other(), TransactionManifest.sample.other())
        assertNotEquals(TransactionManifest.sample(), TransactionManifest.sample.other())

        val instructionsString = """
        CALL_METHOD
            Address("account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease")
            "lock_fee"
            Decimal("0.61")
        ;
        CALL_METHOD
            Address("account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease")
            "withdraw"
            Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
            Decimal("1337")
        ;
        TAKE_FROM_WORKTOP
            Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
            Decimal("1337")
            Bucket("bucket1")
        ;
        CALL_METHOD
            Address("account_rdx16yf8jxxpdtcf4afpj5ddeuazp2evep7quuhgtq28vjznee08master")
            "try_deposit_or_abort"
            Bucket("bucket1")
            Enum<0u8>()
        ;
        
        """.trimIndent()

        assertEquals(TransactionManifest.sample().string, instructionsString)

        val manifest = TransactionManifest.init(
            instructionsString = instructionsString,
            networkId = NetworkId.MAINNET,
            blobs = Blobs.init()
        )

        assertEquals(TransactionManifest.sample(), manifest)
    }

}