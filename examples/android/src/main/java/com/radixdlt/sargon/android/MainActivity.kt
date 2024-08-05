@file:OptIn(ExperimentalMaterial3Api::class)

package com.radixdlt.sargon.android

import android.os.Bundle
import androidx.activity.compose.setContent
import androidx.activity.viewModels
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Button
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.fragment.app.FragmentActivity
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import com.radixdlt.sargon.android.ui.theme.SargonAndroidTheme
import com.radixdlt.sargon.extensions.errorMessage
import com.radixdlt.sargon.os.driver.BiometricsHandler
import dagger.hilt.android.AndroidEntryPoint
import javax.inject.Inject

@AndroidEntryPoint
class MainActivity : FragmentActivity() {

    private val viewModel: MainViewModel by viewModels()

    @Inject
    lateinit var biometricsHandler: BiometricsHandler

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        biometricsHandler.register(this)

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

    val context = LocalContext.current
    Scaffold(
        modifier = modifier,
        topBar = { TopAppBar(title = { Text(text = "Sargon Os") }) },
    ) { padding ->
        Column(modifier = Modifier.padding(padding)) {
            when (val sargonState = state.sargonState) {
                SargonOsManager.SargonState.Idle -> {
                    Text(text = "OS is idle")
                }
                is SargonOsManager.SargonState.Booted -> {
                    Text(text = "OS Booted!")
                }
                is SargonOsManager.SargonState.BootError -> {
                    Text(text = "Os Boot Error")
                    Text(text = sargonState.error.errorMessage)

                    Button(
                        modifier = Modifier.fillMaxWidth(),
                        onClick = { viewModel.retryBooting() }
                    ) {
                        Text(text = "Retry biometrics")
                    }
                }
            }
        }
    }
}