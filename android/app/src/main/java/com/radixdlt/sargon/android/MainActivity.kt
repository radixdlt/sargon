package com.radixdlt.sargon.android

import android.os.Bundle
import android.util.Log
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Button
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import com.radixdlt.sargon.AppearanceId
import com.radixdlt.sargon.DisplayName
import com.radixdlt.sargon.Profile
import com.radixdlt.sargon.SecureStorage
import com.radixdlt.sargon.Wallet
import com.radixdlt.sargon.WalletClientModel
import com.radixdlt.sargon.android.ui.theme.SargonAndroidTheme
import com.radixdlt.sargon.newAppearanceIdPlaceholder
import com.radixdlt.sargon.newAppearanceIdPlaceholderOther
import com.radixdlt.sargon.newDisplayName
import com.radixdlt.sargon.newProfilePlaceholder
import kotlin.random.Random

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        val storage = EphemeralKeystore()

        setContent {
            SargonAndroidTheme {
                WalletContent(storage = storage)
            }
        }
    }
}

@Composable
fun WalletContent(
    modifier: Modifier = Modifier,
    storage: SecureStorage
) {
    var walletState: Wallet? by remember { mutableStateOf(null) }

    Scaffold(modifier = modifier) { padding ->
        Column(modifier = Modifier.padding(padding)) {
            if (walletState == null) {
                Button(
                    onClick = {
                        walletState = Wallet.with(
                            entropy = ByteArray(32) { 0xFF.toByte() },
                            secureStorage = storage
                        )
                    }
                ) {
                    Text(text = "New Wallet")
                }
            }

            walletState?.let { wallet ->

                Log.d("Bakos", storage.toString())
            }
        }
    }

}

val Profile.Companion.placeholder: Profile
    get() = newProfilePlaceholder()

fun DisplayName.Companion.from(value: String): DisplayName {
    return newDisplayName(name = value)
}

val AppearanceId.Companion.placeholder: AppearanceId
    get() = newAppearanceIdPlaceholder()

val AppearanceId.Companion.placeholderOther: AppearanceId
    get() = newAppearanceIdPlaceholderOther()

val Wallet.Companion.defaultPhoneName: String
    get() = "Android Phone"

fun Wallet.Companion.with(
    entropy: ByteArray = ByteArray(32).apply { Random.nextBytes(this) },
    phoneName: String = Wallet.Companion.defaultPhoneName,
    secureStorage: SecureStorage
): Wallet {
    return Wallet.byCreatingNewProfileAndSecretsWithEntropy(
        entropy = entropy,
        walletClientModel = WalletClientModel.ANDROID,
        walletClientName = phoneName,
        secureStorage = secureStorage
    )
}

@Preview(showBackground = true)
@Composable
fun GreetingPreview() {
    SargonAndroidTheme {
//        Greeting("Android")
    }
}