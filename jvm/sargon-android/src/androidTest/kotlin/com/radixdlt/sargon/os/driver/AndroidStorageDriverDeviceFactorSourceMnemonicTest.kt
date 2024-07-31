package com.radixdlt.sargon.os.driver

import android.content.Context
import androidx.test.core.app.ApplicationProvider
import androidx.test.ext.junit.runners.AndroidJUnit4
import androidx.test.filters.MediumTest
import com.radixdlt.sargon.CommonException
import com.radixdlt.sargon.Exactly32Bytes
import com.radixdlt.sargon.FactorSourceIdFromHash
import com.radixdlt.sargon.FactorSourceKind
import com.radixdlt.sargon.MnemonicWithPassphrase
import com.radixdlt.sargon.SecureStorageKey
import com.radixdlt.sargon.mnemonicWithPassphraseToJsonBytes
import com.radixdlt.sargon.newMnemonicWithPassphraseFromJsonBytes
import com.radixdlt.sargon.os.driver.AndroidStorageDriverTest.Companion.sut
import com.radixdlt.sargon.os.storage.EncryptionHelper
import com.radixdlt.sargon.samples.sample
import io.mockk.every
import io.mockk.mockkObject
import io.mockk.slot
import io.mockk.unmockkObject
import junit.framework.TestCase.assertEquals
import junit.framework.TestCase.assertNull
import kotlinx.coroutines.test.runTest
import org.junit.After
import org.junit.Assert.assertTrue
import org.junit.Test
import org.junit.runner.RunWith
import java.io.File

@RunWith(AndroidJUnit4::class)
@MediumTest
class AndroidStorageDriverDeviceFactorSourceMnemonicTest {

    private val testContext: Context = ApplicationProvider.getApplicationContext()

    @After
    fun deleteDatastores() {
        File(testContext.filesDir, "datastore").deleteRecursively()
        mockUnauthorize()
    }

    @Test
    fun testWriteWithNoAuthorization() = runTest {
        val sut = sut(testContext, backgroundScope)

        val id = FactorSourceIdFromHash(
            kind = FactorSourceKind.DEVICE,
            body = Exactly32Bytes.sample()
        )
        val mnemonic = MnemonicWithPassphrase.sample()

        runCatching {
            sut.saveData(
                SecureStorageKey.DeviceFactorSourceMnemonic(id),
                mnemonicWithPassphraseToJsonBytes(mnemonic)
            )
        }.onFailure { error ->
            assertTrue(
                "Expected CommonException.SecureStorageWriteException but got $error",
                error is CommonException.SecureStorageWriteException
            )
        }.onSuccess {
            error("Save operation did not throw when it should.")
        }

    }

    @Test
    fun testWriteWithAuthorization() = runTest {
        val sut = sut(testContext, backgroundScope)

        val id = FactorSourceIdFromHash(
            kind = FactorSourceKind.DEVICE,
            body = Exactly32Bytes.sample()
        )
        val mnemonic = MnemonicWithPassphrase.sample()

        mockAuthorize()
        sut.saveData(
            SecureStorageKey.DeviceFactorSourceMnemonic(id),
            mnemonicWithPassphraseToJsonBytes(mnemonic)
        )

        val retrievedMnemonic = sut.loadData(SecureStorageKey.DeviceFactorSourceMnemonic(id))?.let {
            newMnemonicWithPassphraseFromJsonBytes(it)
        }

        assertEquals(
            mnemonic,
            retrievedMnemonic
        )
    }

    @Test
    fun testRemove() = runTest {
        val sut = sut(testContext, backgroundScope)

        val id = FactorSourceIdFromHash(
            kind = FactorSourceKind.DEVICE,
            body = Exactly32Bytes.sample()
        )
        val mnemonic = MnemonicWithPassphrase.sample()

        mockAuthorize()
        sut.saveData(
            SecureStorageKey.DeviceFactorSourceMnemonic(id),
            mnemonicWithPassphraseToJsonBytes(mnemonic)
        )

        // No need to authorize biometrics in order to remove a mnemonic
        mockUnauthorize()
        sut.deleteDataForKey(SecureStorageKey.DeviceFactorSourceMnemonic(id))

        // Again no need to authorize since there is no data to decrypt
        assertNull(
            sut.loadData(SecureStorageKey.DeviceFactorSourceMnemonic(id))
        )
    }


    private fun mockAuthorize() {
        mockkObject(EncryptionHelper).apply {
            val inputToEncryptSlot = slot<ByteArray>()
            every { EncryptionHelper.encrypt(capture(inputToEncryptSlot), any()) } answers {
                Result.success(inputToEncryptSlot.captured)
            }
            val inputToDecryptSlot = slot<ByteArray>()
            every { EncryptionHelper.decrypt(capture(inputToDecryptSlot), any()) } answers {
                Result.success(inputToDecryptSlot.captured)
            }
        }
    }

    private fun mockUnauthorize() {
        unmockkObject(EncryptionHelper)
    }
}