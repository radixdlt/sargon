package com.radixdlt.sargon.os.driver

import android.os.Build
import androidx.biometric.BiometricManager
import androidx.biometric.BiometricPrompt
import androidx.core.content.ContextCompat
import androidx.fragment.app.FragmentActivity
import androidx.lifecycle.Lifecycle
import androidx.lifecycle.lifecycleScope
import androidx.lifecycle.repeatOnLifecycle
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.channels.Channel
import kotlinx.coroutines.flow.collectLatest
import kotlinx.coroutines.flow.receiveAsFlow
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import kotlinx.coroutines.withTimeout
import timber.log.Timber
import kotlin.coroutines.resume
import kotlin.coroutines.suspendCoroutine

internal interface BiometricAuthorizationDriver {

    suspend fun authorize(): Result<Unit>

}

sealed class BiometricsFailure(override val message: String?) : Exception() {

    data class AuthenticationNotPossible(
        val authenticationStatus: Int
    ) : BiometricsFailure(
        message = "Biometrics failed to request. canAuthenticate() returned [$authenticationStatus] ${authenticationStatus.toAuthenticationStatusMessage()}"
    )

    data class AuthenticationError(
        val errorCode: Int,
        val errorMessage: String
    ) : BiometricsFailure(
        message = "User did not authorize. Received [$errorCode]: $errorMessage"
    )

    companion object {
        private fun Int.toAuthenticationStatusMessage(): String = when (this) {
            BiometricManager.BIOMETRIC_SUCCESS ->
                "The user can successfully authenticate."

            BiometricManager.BIOMETRIC_STATUS_UNKNOWN ->
                "Unable to determine whether the user can authenticate."

            BiometricManager.BIOMETRIC_ERROR_UNSUPPORTED ->
                "The user can't authenticate because the specified options are incompatible with the current Android version."

            BiometricManager.BIOMETRIC_ERROR_HW_UNAVAILABLE ->
                "The user can't authenticate because the hardware is unavailable. Try again later."

            BiometricManager.BIOMETRIC_ERROR_NONE_ENROLLED ->
                "The user can't authenticate because no biometric or device credential is enrolled."

            BiometricManager.BIOMETRIC_ERROR_NO_HARDWARE ->
                "The user can't authenticate because there is no suitable hardware (e. g. no biometric sensor or no keyguard)."

            BiometricManager.BIOMETRIC_ERROR_SECURITY_UPDATE_REQUIRED ->
                "The user can't authenticate because a security vulnerability has been discovered with one or more hardware sensors. The affected sensor(s) are unavailable until a security update has addressed the issue."

            else -> ""
        }
    }

}

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
        activity.lifecycleScope.launch {
            // Listen to biometric prompt requests while the activity is at least started.
            activity.lifecycle.repeatOnLifecycle(Lifecycle.State.STARTED) {
                biometricRequestsChannel.receiveAsFlow().collectLatest {
                    val result = requestBiometricsAuthorization(activity)

                    // Send back the result to sargon os
                    biometricsResultsChannel.send(result)
                }
            }
        }
    }

    internal suspend fun askForBiometrics(): Result<Unit> {
        // Suspend until an activity is subscribed to this channel
        withTimeout(5000) {
            biometricRequestsChannel.send(Unit)
        }

        // If an activity is already registered, then we need to wait until the user provides
        // the response from the biometrics prompt
        return biometricsResultsChannel.receive()
    }

    private suspend fun requestBiometricsAuthorization(
        activity: FragmentActivity
    ): Result<Unit> = withContext(Dispatchers.Main) {
        suspendCoroutine { continuation ->
            val biometricManager = BiometricManager.from(activity)

            val authenticationPreCheckStatus =
                biometricManager.canAuthenticate(allowedAuthenticators)
            if (authenticationPreCheckStatus != BiometricManager.BIOMETRIC_SUCCESS) {
                continuation.resume(
                    Result.failure(
                        BiometricsFailure.AuthenticationNotPossible(authenticationPreCheckStatus)
                    )
                )
                return@suspendCoroutine
            }

            val authCallback = object : BiometricPrompt.AuthenticationCallback() {
                override fun onAuthenticationSucceeded(result: BiometricPrompt.AuthenticationResult) {
                    continuation.resume(Result.success(Unit))
                }

                override fun onAuthenticationError(errorCode: Int, errString: CharSequence) {
                    continuation.resume(
                        Result.failure(
                            BiometricsFailure.AuthenticationError(
                                errorCode,
                                errString.toString()
                            )
                        )
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