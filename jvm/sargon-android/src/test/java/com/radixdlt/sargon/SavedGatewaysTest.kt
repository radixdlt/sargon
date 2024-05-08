package com.radixdlt.sargon

//class SavedGatewaysTest: SampleTestable<SavedGateways> {
//
//    override val samples: List<Sample<SavedGateways>>
//        get() = listOf(SavedGateways.sample)
//
//    @Test
//    fun testNew() {
//        val mainnet = Gateway.sampleMainnet()
//        assertEquals(Gateway.sampleMainnet(), mainnet)
//        val gateways = SavedGateways.init(current = mainnet)
//        assertEquals(NetworkId.MAINNET, gateways.current.network.id)
//    }
//
//    @Test
//    fun testDefault() {
//        assertEquals(
//            SavedGateways(
//                current = Gateway.mainnet,
//                other = Gateways.init(Gateway.stokenet)
//            ),
//            SavedGateways.default
//        )
//    }
//
//    @Test
//    fun testChangeCurrent() {
//        val newGateway = Gateway.init(
//            url = "https://hammunet-gateway.radixdlt.com",
//            networkId = NetworkId.HAMMUNET
//        )
//        val gateways = SavedGateways.default.changeCurrent(newCurrent = newGateway)
//
//        assertEquals(
//            SavedGateways(
//                current = newGateway,
//                other = Gateways.init(Gateway.stokenet, Gateway.mainnet)
//            ),
//            gateways
//        )
//
//        assertEquals(
//            listOf(newGateway, Gateway.stokenet, Gateway.mainnet),
//            gateways.all
//        )
//    }
//}