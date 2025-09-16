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
import com.radixdlt.sargon.CommonException
import com.radixdlt.sargon.SecureStorageAccessErrorKind
import com.radixdlt.sargon.SecureStorageKey
import com.radixdlt.sargon.secureStorageKeyIdentifier
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

class BiometricsFailure(
    @AuthenticationError
    val errorCode: Int,
    val errorMessage: String?
) : Exception("[$errorCode] $errorMessage") {

    fun toCommonException(
        key: SecureStorageKey
    ): CommonException = CommonException.SecureStorageAccessException(
        key = secureStorageKeyIdentifier(key),
        errorKind = when (errorCode) {
            BiometricPrompt.ERROR_CANCELED -> SecureStorageAccessErrorKind.CANCELLED
            BiometricPrompt.ERROR_HW_NOT_PRESENT -> SecureStorageAccessErrorKind.HARDWARE_NOT_PRESENT
            BiometricPrompt.ERROR_HW_UNAVAILABLE -> SecureStorageAccessErrorKind.HARDWARE_UNAVAILABLE
            BiometricPrompt.ERROR_LOCKOUT -> SecureStorageAccessErrorKind.LOCKOUT
            BiometricPrompt.ERROR_LOCKOUT_PERMANENT -> SecureStorageAccessErrorKind.LOCKOUT_PERMANENT
            BiometricPrompt.ERROR_NEGATIVE_BUTTON -> SecureStorageAccessErrorKind.NEGATIVE_BUTTON
            BiometricPrompt.ERROR_NO_BIOMETRICS -> SecureStorageAccessErrorKind.NO_BIOMETRICS
            BiometricPrompt.ERROR_NO_DEVICE_CREDENTIAL -> SecureStorageAccessErrorKind.NO_DEVICE_CREDENTIAL
            BiometricPrompt.ERROR_NO_SPACE -> SecureStorageAccessErrorKind.NO_SPACE
            BiometricPrompt.ERROR_TIMEOUT -> SecureStorageAccessErrorKind.TIMEOUT
            BiometricPrompt.ERROR_UNABLE_TO_PROCESS -> SecureStorageAccessErrorKind.UNABLE_TO_PROCESS
            BiometricPrompt.ERROR_USER_CANCELED -> SecureStorageAccessErrorKind.USER_CANCELLED
            BiometricPrompt.ERROR_VENDOR -> SecureStorageAccessErrorKind.VENDOR
            else -> throw CommonException.Unknown()
        },
        errorMessage = errorMessage.orEmpty()
    )

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

    fun register(
        activity: FragmentActivity,
        callbacks: OnBiometricsLifecycleCallbacks? = null
    ) {
        biometricRequestsChannel
            .receiveAsFlow()
            .flowWithLifecycle(
                lifecycle = activity.lifecycle,
                minActiveState = Lifecycle.State.STARTED
            )
            .onEach {
                callbacks?.onBeforeBiometricsRequest()
                val result = requestBiometricsAuthorization(activity)

                // Send back the result to sargon os
                biometricsResultsChannel.send(result)
                callbacks?.onAfterBiometricsResult()
            }
            .launchIn(activity.lifecycleScope)
    }

    suspend fun askForBiometrics(): Result<Unit> {
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

/**
 * Callback functions called to notify the host about the lifecycle
 * of a biometrics request
 */
interface OnBiometricsLifecycleCallbacks {
    /**
     * Called right before a biometrics request is about to be appeared.
     */
    fun onBeforeBiometricsRequest()

    /**
     * Called right after a biometrics request has ended with a result.
     */
    fun onAfterBiometricsResult()
}