package com.radixdlt.sargon.os.driver

import android.os.Build
import androidx.biometric.BiometricManager
import androidx.biometric.BiometricPrompt
import androidx.biometric.BiometricPrompt.AuthenticationError
import androidx.core.content.ContextCompat
import androidx.fragment.app.FragmentActivity
import androidx.lifecycle.Lifecycle
import androidx.lifecycle.flowWithLifecycle
import androidx.lifecycle.lifecycleScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.channels.Channel
import kotlinx.coroutines.flow.launchIn
import kotlinx.coroutines.flow.onEach
import kotlinx.coroutines.flow.receiveAsFlow
import kotlinx.coroutines.withContext
import timber.log.Timber
import kotlin.coroutines.resume
import kotlin.coroutines.suspendCoroutine

internal interface BiometricAuthorizationDriver {

    suspend fun authorize(): Result<Unit>

}

internal class BiometricsFailure(
    @AuthenticationError
    val errorCode: Int,
    val errorMessage: String?
) : Exception("[$errorCode] $errorMessage")

internal class AndroidBiometricAuthorizationDriver(
    private val biometricsHandler: BiometricsHandler
) : BiometricAuthorizationDriver {


    override suspend fun authorize(): Result<Unit> = biometricsHandler.askForBiometrics()

}

class BiometricsHandler(
    internal val biometricsSystemDialogTitle: String
) {

    private val biometricRequestsChannel = Channel<Unit>()
    private val biometricsResultsChannel = Channel<Result<Unit>>()

    fun register(activity: FragmentActivity) {
        biometricRequestsChannel
            .receiveAsFlow()
            .flowWithLifecycle(
                lifecycle = activity.lifecycle,
                minActiveState = Lifecycle.State.STARTED
            )
            .onEach {
                val result = requestBiometricsAuthorization(activity)

                // Send back the result to sargon os
                biometricsResultsChannel.send(result)
            }
            .launchIn(activity.lifecycleScope)
    }

    internal suspend fun askForBiometrics(): Result<Unit> {
        // Suspend until an activity is subscribed to this channel and is at least started
        biometricRequestsChannel.send(Unit)

        // If an activity is already registered, then we need to wait until the user provides
        // the response from the biometrics prompt
        return biometricsResultsChannel.receive()
    }

    private suspend fun requestBiometricsAuthorization(
        activity: FragmentActivity
    ): Result<Unit> = withContext(Dispatchers.Main) {
        suspendCoroutine { continuation ->
            val authCallback = object : BiometricPrompt.AuthenticationCallback() {
                override fun onAuthenticationSucceeded(result: BiometricPrompt.AuthenticationResult) {
                    continuation.resume(Result.success(Unit))
                }

                override fun onAuthenticationError(errorCode: Int, errString: CharSequence) {
                    continuation.resume(
                        Result.failure(BiometricsFailure(errorCode, errString.toString()))
                    )
                }

                override fun onAuthenticationFailed() {
                    Timber.tag("Sargon").w("Biometrics failed.")
                }
            }

            val promptInfo = BiometricPrompt.PromptInfo.Builder()
                .setTitle(biometricsSystemDialogTitle)
                .setAllowedAuthenticators(allowedAuthenticators)
                .build()

            val biometricPrompt = BiometricPrompt(
                activity,
                ContextCompat.getMainExecutor(activity),
                authCallback
            )

            biometricPrompt.authenticate(promptInfo)
        }
    }

    private val allowedAuthenticators = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.R) {
        BiometricManager.Authenticators.BIOMETRIC_STRONG or
                BiometricManager.Authenticators.DEVICE_CREDENTIAL
    } else {
        BiometricManager.Authenticators.BIOMETRIC_WEAK or
                BiometricManager.Authenticators.DEVICE_CREDENTIAL
    }
}