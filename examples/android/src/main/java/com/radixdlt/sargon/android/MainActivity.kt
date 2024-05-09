@file:OptIn(ExperimentalMaterial3Api::class)

package com.radixdlt.sargon.android

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.text.KeyboardActions
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material3.Button
import androidx.compose.material3.ElevatedCard
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TextField
import androidx.compose.material3.TopAppBar
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.input.ImeAction
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import com.radixdlt.sargon.DisplayName
import com.radixdlt.sargon.NetworkId
import com.radixdlt.sargon.NonEmptyMax32Bytes
import com.radixdlt.sargon.Profile
import com.radixdlt.sargon.ProfileNetwork
import com.radixdlt.sargon.SecureStorageDriver
import com.radixdlt.sargon.Wallet
import com.radixdlt.sargon.WalletClientModel
import com.radixdlt.sargon.android.ui.theme.SargonAndroidTheme
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.extensions.toBagOfBytes
import com.radixdlt.sargon.samples.sample
import kotlin.random.Random

class MainActivity : ComponentActivity() {

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        val storage = EphemeralKeystore()

        setContent { SargonAndroidTheme { WalletContent(storage = storage) } }
    }
}

@Composable
fun WalletContent(modifier: Modifier = Modifier, storage: SecureStorageDriver) {
    var walletState: Wallet? by remember { mutableStateOf(null) }
    var profile: Profile? by remember { mutableStateOf(null) }

    Scaffold(
            modifier = modifier,
            topBar = { TopAppBar(title = { Text(text = "Wallet Test") }) },
            bottomBar = {
                if (walletState == null) {
                    Button(
                            modifier = Modifier
                                .padding(16.dp)
                                .fillMaxWidth(),
                            onClick = {
                                walletState =
                                        Wallet.with(
                                                entropy = ByteArray(32) { 0xFF.toByte() },
                                                secureStorage = storage
                                        )
                                profile = walletState?.profile()
                            }
                    ) { Text(text = "Generate new Wallet") }
                } else if (profile?.networks?.isEmpty() == true) {
                    Column(modifier = Modifier.padding(16.dp)) {
                        var accountName by remember { mutableStateOf("") }
                        TextField(
                                modifier = Modifier.fillMaxWidth(),
                                value = accountName,
                                onValueChange = { accountName = it },
                                label = { Text(text = "New Account Name") },
                                singleLine = true,
                                keyboardOptions = KeyboardOptions(imeAction = ImeAction.Done),
                                keyboardActions =
                                        KeyboardActions(
                                                onDone = {
                                                    walletState?.createAndSaveNewAccount(
                                                            networkId = NetworkId.MAINNET,
                                                            name = DisplayName(accountName)
                                                    )

                                                    profile = walletState?.profile()
                                                }
                                        )
                        )
                    }
                }
            }
    ) { padding ->
        LazyColumn(modifier = Modifier.padding(padding), contentPadding = PaddingValues(16.dp)) {
            items(profile?.networks.orEmpty()) {
                Network(
                        network = it,
                        onAccountAdd = { newName ->
                            walletState?.createNewAccount(NetworkId.MAINNET, DisplayName(newName))
                                    ?.let {
                                        walletState?.addAccount(it)

                                        profile = walletState?.profile()
                                    }
                        }
                )
            }
        }
    }
}

@Composable
fun Network(
    modifier: Modifier = Modifier,
    network: ProfileNetwork,
    onAccountAdd: (String) -> Unit
) {
    ElevatedCard(modifier = modifier.fillMaxWidth()) {
        Spacer(modifier = Modifier.height(16.dp))
        Text(modifier = Modifier.padding(horizontal = 16.dp), text = "${network.id}")

//        network.accounts.forEach { account ->
//            Text(
//                    modifier = Modifier.padding(horizontal = 32.dp),
//                    text = account.displayName.value,
//                    style = MaterialTheme.typography.labelLarge
//            )
//            Text(
//                    modifier = Modifier.padding(horizontal = 32.dp),
//                    text = account.address.string,
//                    style = MaterialTheme.typography.labelSmall
//            )
//            HorizontalDivider(modifier = Modifier.padding(horizontal = 32.dp))
//        }

        Column(modifier = Modifier.padding(16.dp)) {
            var newAccountName by remember { mutableStateOf("") }
            TextField(
                    modifier = Modifier.fillMaxWidth(),
                    value = newAccountName,
                    onValueChange = { newAccountName = it },
                    singleLine = true,
                    keyboardOptions = KeyboardOptions(imeAction = ImeAction.Done),
                    keyboardActions =
                            KeyboardActions(
                                    onDone = {
                                        onAccountAdd(newAccountName)
                                        newAccountName = ""
                                    }
                            )
            )
        }
    }
}

val Wallet.Companion.defaultPhoneName: String
    get() = "Android Phone"

fun Wallet.Companion.with(
        entropy: ByteArray = ByteArray(32).apply { Random.nextBytes(this) },
        phoneName: String = Wallet.Companion.defaultPhoneName,
        secureStorage: SecureStorageDriver
): Wallet {
    return Wallet.byCreatingNewProfileAndSecretsWithEntropy(
            entropy = NonEmptyMax32Bytes(entropy.toBagOfBytes()),
            walletClientModel = WalletClientModel.ANDROID,
            walletClientName = phoneName,
            secureStorage = secureStorage
    )
}

@OptIn(UsesSampleValues::class)
@Preview(showBackground = true)
@Composable
fun NetworkPreview() {
    val profile = Profile.sample()
    Network(network = profile.networks.first(), onAccountAdd = {})
}
