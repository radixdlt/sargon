package com.radixdlt.sargon.serializer

import com.radixdlt.sargon.BagOfBytes
import com.radixdlt.sargon.extensions.bagOfBytes
import com.radixdlt.sargon.extensions.string
import com.radixdlt.sargon.samples.Sample
import kotlinx.serialization.KSerializer
import kotlinx.serialization.builtins.ListSerializer
import kotlinx.serialization.json.Json
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Nested
import org.junit.jupiter.api.Test

/**
 * A tester class that verifies the implementation of custom serializers that aim
 * to bridge the serialization between Rust (serde) and Kotlinx serialization. By using these
 * serializers we can essentially use Rust's serde implementation directly in kotlinx-serialization.
 */
abstract class KotlinRustSerializerTest<TYPE, SERIALIZER>(
    val sample: Sample<TYPE>,
    val serializer: SERIALIZER
) where SERIALIZER: KSerializer<TYPE> {

    abstract fun rustFunctionFromJsonBytes(
        jsonBytes: BagOfBytes
    ): TYPE

    abstract fun rustFunctionToJsonBytes(
        value: TYPE
    ): BagOfBytes

    @Nested
    inner class SingleValueTests {
        @Test
        fun testSerializeKotlinDeserializeRust() {
            val sampleValue = sample()

            val json = Json.encodeToString(
                serializer,
                sampleValue
            )

            assertEquals(
                sampleValue,
                rustFunctionFromJsonBytes(bagOfBytes(json))
            )
        }

        @Test
        fun testSerializeRustDeserializeKotlin() {
            val sampleValue = sample()

            val rustJson = rustFunctionToJsonBytes(sampleValue).string

            assertEquals(
                sampleValue,
                Json.decodeFromString(
                    serializer,
                    rustJson
                )
            )
        }

        @Test
        fun testRoundtrip() {
            val sampleValue = sample()

            val json = Json.encodeToString(
                serializer,
                sampleValue
            )

            assertEquals(
                sampleValue,
                Json.decodeFromString(
                    serializer,
                    json
                )
            )
        }
    }

    @Nested
    inner class CollectionTests {

        @Test
        fun testRoundtrip() {
            val sampleValues = sample.all

            val json = Json.encodeToString(
                ListSerializer(elementSerializer = serializer),
                sampleValues
            )

            assertEquals(
                sampleValues,
                Json.decodeFromString(
                    ListSerializer(elementSerializer = serializer),
                    json
                )
            )
        }
    }

}