@file:OptIn(ExperimentalMaterial3Api::class, UsesSampleValues::class)

package com.radixdlt.sargon.android

import android.os.Bundle
import androidx.activity.compose.setContent
import androidx.activity.viewModels
import androidx.compose.animation.AnimatedVisibility
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.text.KeyboardActions
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.Button
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Switch
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.material3.TopAppBar
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.input.ImeAction
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import androidx.fragment.app.FragmentActivity
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import com.radixdlt.sargon.CommonException
import com.radixdlt.sargon.FactorSource
import com.radixdlt.sargon.NetworkId
import com.radixdlt.sargon.Profile
import com.radixdlt.sargon.ProfileState
import com.radixdlt.sargon.android.ui.theme.SargonAndroidTheme
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.extensions.errorCode
import com.radixdlt.sargon.extensions.errorMessage
import com.radixdlt.sargon.extensions.isMain
import com.radixdlt.sargon.extensions.kind
import com.radixdlt.sargon.extensions.name
import com.radixdlt.sargon.extensions.string
import com.radixdlt.sargon.extensions.vendor
import com.radixdlt.sargon.extensions.version
import com.radixdlt.sargon.os.SargonOsState
import com.radixdlt.sargon.os.driver.BiometricsHandler
import com.radixdlt.sargon.os.driver.OnBiometricsLifecycleCallbacks
import com.radixdlt.sargon.samples.sample
import dagger.hilt.android.AndroidEntryPoint
import timber.log.Timber
import javax.inject.Inject

@AndroidEntryPoint
class MainActivity : FragmentActivity() {

    private val viewModel: MainViewModel by viewModels()

    @Inject
    lateinit var biometricsHandler: BiometricsHandler

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        biometricsHandler.register(this, object : OnBiometricsLifecycleCallbacks {
            override fun onBeforeBiometricsRequest() {
                Timber.d("About to request biometrics")
            }

            override fun onAfterBiometricsResult() {
                Timber.d("Biometrics request ended.")
            }

        })

        setContent {
            SargonAndroidTheme {
                WalletContent(viewModel = viewModel)
            }
        }
    }
}

@Composable
fun WalletContent(
    modifier: Modifier = Modifier,
    viewModel: MainViewModel
) {
    val state: MainViewModel.State by viewModel.state.collectAsStateWithLifecycle()

    Scaffold(
        modifier = modifier,
        topBar = {
            TopAppBar(
                title = {
                    Column {
                        Text(text = "Sargon App")
                        val status = when (val sargonState = state.sargonState) {
                            SargonOsState.Idle -> "Idle"
                            is SargonOsState.Booted -> "Booted"
                            is SargonOsState.BootError -> "Boot Error\n${sargonState.error.printStackTrace()}"
                        }
                        Text(
                            text = "OS Status: $status",
                            style = MaterialTheme.typography.labelMedium
                        )
                    }

                }
            )
        },
        bottomBar = {
            if (state.profileState is ProfileState.Loaded) {
                Button(
                    modifier = Modifier
                        .fillMaxWidth()
                        .padding(16.dp),
                    onClick = { viewModel.onDeleteWallet() }
                ) {
                    Text(text = "Delete wallet")
                }
            }
        }
    ) { padding ->
        Column(
            modifier = Modifier
                .padding(padding)
                .fillMaxSize()
        ) {
            state.info?.let { info ->
                var isHostInfoVisible by remember {
                    mutableStateOf(false)
                }

                TextButton(
                    onClick = { isHostInfoVisible = !isHostInfoVisible }
                ) {
                    Text(text = "Host information (ℹ)")
                }

                AnimatedVisibility(visible = isHostInfoVisible) {
                    Column(modifier = Modifier.padding(horizontal = 16.dp)) {
                        Text(
                            text = "Host ID",
                            style = MaterialTheme.typography.titleMedium
                        )
                        Text(
                            text = "\t${info.id.id}"
                        )

                        Text(
                            text = "Host description",
                            style = MaterialTheme.typography.titleMedium
                        )
                        Text(
                            text = "\t${info.info.description.string}"
                        )

                        Text(
                            text = "Host OS",
                            style = MaterialTheme.typography.titleMedium
                        )
                        Text(
                            text = "\t${info.info.hostOs.name} - ${info.info.hostOs.vendor} - ${info.info.hostOs.version}"
                        )

                        Text(
                            text = "App Version",
                            style = MaterialTheme.typography.titleMedium
                        )
                        Text(
                            text = "\t${info.info.hostAppVersion}"
                        )
                    }
                }

                HorizontalDivider()
            }

            when (val profileState = state.profileState) {
                null -> CircularProgressIndicator(
                    modifier = Modifier
                        .padding(32.dp)
                        .align(Alignment.CenterHorizontally)
                )

                is ProfileState.None -> NoProfileContent(
                    modifier = Modifier
                        .padding(16.dp),
                    onCreateNewWallet = viewModel::onCreateNewWallet,
                    onImportWallet = {
                        viewModel.onImportWallet(Profile.sample())
                    }
                )

                is ProfileState.Incompatible -> IncompatibleProfile(
                    modifier = Modifier
                        .padding(16.dp),
                    error = profileState.v1
                )

                is ProfileState.Loaded -> ProfileContent(
                    modifier = Modifier
                        .padding(16.dp),
                    profile = profileState.v1,
                    onDevModeChanged = { enabled ->
                        viewModel.onDevModeChanged(enabled)
                    },
                    onAccountAdded = { networkId, accountName ->
                        viewModel.onCreateAccountWithDevice(
                            networkId = networkId,
                            accountName = accountName
                        )
                    }
                )
            }
        }
    }
}

@Composable
private fun NoProfileContent(
    modifier: Modifier = Modifier,
    onCreateNewWallet: () -> Unit,
    onImportWallet: () -> Unit,
) {
    Column(modifier = modifier) {
        Text(
            modifier = Modifier
                .fillMaxWidth()
                .padding(bottom = 16.dp),
            text = "Welcome to Sargon OS",
            style = MaterialTheme.typography.titleMedium,
            textAlign = TextAlign.Center
        )

        Button(
            modifier = Modifier.fillMaxWidth(),
            onClick = onCreateNewWallet
        ) {
            Text(text = "Create new profile")
        }

        Button(
            modifier = Modifier.fillMaxWidth(),
            onClick = onImportWallet
        ) {
            Column(
                modifier = Modifier.fillMaxWidth(),
                horizontalAlignment = Alignment.CenterHorizontally
            ) {
                Text(text = "Import sample profile")
                Text(
                    text = "(Skips Main Seed Phrase)",
                    style = MaterialTheme.typography.bodySmall
                )
            }

        }
    }

}

@Composable
private fun IncompatibleProfile(
    modifier: Modifier = Modifier,
    error: CommonException
) {
    Text(
        modifier = modifier,
        text = "Error loading profile: [${error.errorCode}]  - ${error.errorMessage}"
    )
}

@Composable
private fun ProfileContent(
    modifier: Modifier = Modifier,
    profile: Profile,
    onDevModeChanged: (Boolean) -> Unit,
    onAccountAdded: (networkId: NetworkId, accountName: String) -> Unit
) {
    Column(
        modifier = modifier
            .fillMaxSize()
            .verticalScroll(rememberScrollState())
    ) {
        Text(
            text = "Profile",
            style = MaterialTheme.typography.titleMedium
        )
        Text(text = "\t${profile.header.id}")

        Text(
            text = "Last Update:",
            style = MaterialTheme.typography.titleMedium
        )
        Text(text = "\t${profile.header.lastModified}")

        Text(
            text = "Created:",
            style = MaterialTheme.typography.titleMedium
        )
        Text(text = "\t${profile.header.creatingDevice.description}\n\t${profile.header.creatingDevice.date}")

        Text(
            text = "Imported:",
            style = MaterialTheme.typography.titleMedium
        )
        Text(text = "\t${profile.header.lastUsedOnDevice.description}\n\t${profile.header.lastUsedOnDevice.date}")

        Row(
            modifier = Modifier.fillMaxWidth(),
            verticalAlignment = Alignment.CenterVertically,
            horizontalArrangement = Arrangement.SpaceBetween
        ) {
            Text(
                text = "Dev Mode:",
                style = MaterialTheme.typography.titleMedium
            )

            Switch(
                checked = profile.appPreferences.security.isDeveloperModeEnabled,
                onCheckedChange = onDevModeChanged
            )
        }

        Text(
            text = "Factor Sources:",
            style = MaterialTheme.typography.titleMedium
        )
        profile.factorSources.forEach { fs ->
            val fsHint = when (fs) {
                is FactorSource.ArculusCard -> "${fs.value.hint.label} ${fs.value.hint.model}"
                is FactorSource.Device -> "${fs.value.hint.label} ${fs.value.hint.model}"
                is FactorSource.Ledger -> "${fs.value.hint.label} ${fs.value.hint.model}"
                is FactorSource.OffDeviceMnemonic -> "${fs.value.hint.label}"
                is FactorSource.Password -> fs.value.hint.label
            }

            val kind = if (fs is FactorSource.Device) {
                val main = if (fs.isMain) "MAIN" else ""
                "[${fs.kind} $main]"
            } else {
                "[${fs.kind}]"
            }
            Text(text = "• $kind - $fsHint")
        }
        Text(
            text = "Networks:",
            style = MaterialTheme.typography.titleMedium
        )
        profile.networks.forEach { network ->
            Text(
                modifier = Modifier.padding(top = 8.dp),
                text = "- ${network.id}",
                style = MaterialTheme.typography.titleSmall
            )

            Text(
                modifier = Modifier.padding(horizontal = 16.dp),
                text = "Accounts:",
                style = MaterialTheme.typography.titleSmall
            )
            network.accounts.forEach { account ->
                Text(
                    modifier = Modifier.padding(horizontal = 16.dp),
                    text = "• ${account.displayName.value}"
                )
            }
            var newAccountName by remember {
                mutableStateOf("")
            }
            OutlinedTextField(
                modifier = Modifier.fillMaxWidth(),
                value = newAccountName,
                onValueChange = {
                    newAccountName = it
                },
                placeholder = {
                    Text(text = "New Account name...")
                },
                maxLines = 1,
                keyboardActions = KeyboardActions(
                    onDone = {
                        if (newAccountName.isBlank()) return@KeyboardActions

                        onAccountAdded(
                            network.id,
                            newAccountName
                        )

                        newAccountName = ""
                    }
                ),
                keyboardOptions = KeyboardOptions(
                    imeAction = ImeAction.Done
                )
            )

            Text(
                modifier = Modifier.padding(horizontal = 16.dp),
                text = "Personas:",
                style = MaterialTheme.typography.titleSmall
            )
            network.personas.forEach { persona ->
                Text(
                    modifier = Modifier.padding(horizontal = 16.dp),
                    text = "• ${persona.displayName.value}"
                )
            }
        }

    }

}