package com.radixdlt.sargon.annotation

/**
 * User needs to opt in to this annotation in order to use sample values.
 *
 * Tests can be configured to automatically opt-in to this feature by adding the following code
 * into the `build.gradle`
 *
 * In Groovy:
 * ```
 * tasks.withType(KotlinCompile).configureEach {
 *     if (it.name.contains("Test")) {
 *         kotlinOptions.freeCompilerArgs += '-Xopt-in=com.radixdlt.sargon.annotation.UsesSampleValues'
 *     }
 * }
 * ```
 * In Kotlin DSL
 * ```
 * tasks.withType<KotlinCompile>().configureEach {
 *     if (name.contains("Test")) {
 *         kotlinOptions.freeCompilerArgs += "-Xopt-in=com.radixdlt.sargon.annotation.UsesSampleValues"
 *     }
 * }
 * ```
 *
 * When in need to be used in composable previews the only solution for now is to use it like this:
 * ```
 * @OptIn(UsesSampleValues::class)
 * @Preview
 * @Composable
 * fun SomePreview() {
 *    val sampleValue = SomeClass.sample()
 *    ...
 * }
 * ```
 */
@RequiresOptIn
@Retention(AnnotationRetention.BINARY)
annotation class UsesSampleValues
