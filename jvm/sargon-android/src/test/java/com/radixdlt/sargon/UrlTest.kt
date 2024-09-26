package com.radixdlt.sargon

import android.net.Uri
import android.util.Size
import androidx.core.net.toUri
import com.radixdlt.sargon.extensions.intoImageUrl
import com.radixdlt.sargon.extensions.isVectorImage
import com.radixdlt.sargon.extensions.toUrl
import com.radixdlt.sargon.extensions.toUrlOrNull
import io.mockk.every
import io.mockk.mockk
import io.mockk.mockkConstructor
import io.mockk.slot
import okhttp3.HttpUrl.Companion.toHttpUrl
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertNull
import org.junit.jupiter.api.Assertions.assertTrue
import org.junit.jupiter.api.BeforeEach
import org.junit.jupiter.api.Test
import java.net.URI

class UrlTest {

    @Test
    fun testToUrl() {
        assertEquals(
            "https://svgshare.com/i/U7z.svg",
            "https://svgshare.com/i/U7z.svg".toUrl().toString(),
        )
    }

    @Test
    fun testToUrlOrNull() {
        assertEquals(
            "https://svgshare.com/i/U7z.svg",
            "https://svgshare.com/i/U7z.svg".toUrlOrNull().toString(),
        )

        assertNull(
            "data:image/svg+xml,%3Csvg%20viewBox%3D%220%200%201000%201000%22%20".toUrlOrNull(),
        )
    }

    @Test
    fun testVectorImage() {
        val svgUrl = mockUri("https://svgshare.com/i/U7z.svg")

        assertTrue(svgUrl.isVectorImage(imageType = VectorImageType.SVG))
    }

    @Test
    fun testImageUrl() {
        val size = mockSize(width = 1024, height = 1024)
        val imageServiceURL = "https://image-service-dev.extratools.works".toUrl()
        val svgURL = mockUri("https://svgshare.com/i/U7z.svg")

        assertEquals(
            "https://image-service-dev.extratools.works/?imageOrigin=https%3A%2F%2Fsvgshare.com%2Fi%2FU7z.svg&imageSize=1024x1024&format=png",
            svgURL.intoImageUrl(
                imageServiceUrl = imageServiceURL,
                size = size
            ).toString()
        )
    }

    @Test
    fun testImageUrlWithDataUrl() {
        val size = mockSize(width = 1024, height = 1024)
        val imageServiceURL = "https://image-service-dev.extratools.works".toUrl()
        val svgDataUrl = mockUri("data:image/svg+xml,%3Csvg%20viewBox%3D%220%200%201000%201000%22%20" +
                "xmlns%3D%22http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%22%3E%0A%3Cpolygon%20fill%3" +
                "D%22hsla%2890%2C99%25%2C52%25%2C1%29%22%20points%3D%220%2C%200%2C%201000%2C" +
                "%201000%2C%200%2C%201000%22%20transform%3D%22scale%28-1%2C1%29%20translate%" +
                "28-1000%29%22%2F%3E%0A%3Cpolygon%20fill%3D%22hsla%28199%2C90%25%2C64%25%2C1" +
                "%29%22%20points%3D%221000%2C%201000%2C%201000%2C%200%2C%200%2C%200%22%20tra" +
                "nsform%3D%22scale%28-1%2C1%29%20translate%28-1000%29%22%2F%3E%0A%3Cpath%20d" +
                "%3D%22M1000%2C229%20A1000%2C1000%2C0%2C0%2C0%2C229%2C1000%20L1000%2C1000%20" +
                "z%22%20fill%3D%22hsla%28140%2C98%25%2C61%25%2C1%29%22%2F%3E%0A%3Cpath%20d%3" +
                "D%22M392%2C500%20L608%2C500%20M500%2C392%20L500%2C608%22%20stroke%3D%22hsla" +
                "%2847%2C92%25%2C61%25%2C1%29%22%20stroke-width%3D%2272%22%2F%3E%0A%3C%2Fsvg" +
                "%3E")

        assertEquals(
            "https://image-service-dev.extratools.works/?imageOrigin=data%3Aimage%2Fsvg" +
                    "%2Bxml%2C%253Csvg%2520viewBox%253D%25220%25200%25201000%25201000%2522%2520x" +
                    "mlns%253D%2522http%253A%252F%252Fwww.w3.org%252F2000%252Fsvg%2522%253E%250A" +
                    "%253Cpolygon%2520fill%253D%2522hsla%252890%252C99%2525%252C52%2525%252C1%25" +
                    "29%2522%2520points%253D%25220%252C%25200%252C%25201000%252C%25201000%252C%2" +
                    "5200%252C%25201000%2522%2520transform%253D%2522scale%2528-1%252C1%2529%2520" +
                    "translate%2528-1000%2529%2522%252F%253E%250A%253Cpolygon%2520fill%253D%2522" +
                    "hsla%2528199%252C90%2525%252C64%2525%252C1%2529%2522%2520points%253D%252210" +
                    "00%252C%25201000%252C%25201000%252C%25200%252C%25200%252C%25200%2522%2520tr" +
                    "ansform%253D%2522scale%2528-1%252C1%2529%2520translate%2528-1000%2529%2522%" +
                    "252F%253E%250A%253Cpath%2520d%253D%2522M1000%252C229%2520A1000%252C1000%252" +
                    "C0%252C0%252C0%252C229%252C1000%2520L1000%252C1000%2520z%2522%2520fill%253D" +
                    "%2522hsla%2528140%252C98%2525%252C61%2525%252C1%2529%2522%252F%253E%250A%25" +
                    "3Cpath%2520d%253D%2522M392%252C500%2520L608%252C500%2520M500%252C392%2520L5" +
                    "00%252C608%2522%2520stroke%253D%2522hsla%252847%252C92%2525%252C61%2525%252" +
                    "C1%2529%2522%2520stroke-width%253D%252272%2522%252F%253E%250A%253C%252Fsvg%" +
                    "253E&imageSize=1024x1024&format=png",
            svgDataUrl.intoImageUrl(imageServiceUrl = imageServiceURL, size = size).toString()
        )
    }

    private fun mockUri(urlString: String): Uri {
        val uri = mockk<Uri>()
        every { uri.toString() } returns urlString
        return uri
    }

    private fun mockSize(width: Int, height: Int): Size {
        val size = mockk<Size>()
        every { size.width } returns width
        every { size.height } returns height
        return size
    }
}