<script lang="ts" setup>
import { Status, StatusType } from '@injectivelabs/utils'
import type { Token } from '@injectivelabs/token-metadata'
import { UiSpotMarketWithToken } from '@injectivelabs/sdk-ui-ts'
import { BusEvents, Modal, USDCSymbol } from '@/types'

const route = useRoute()
const spotStore = useSpotStore()
const modalStore = useModalStore()
const tokenStore = useTokenStore()
const accountStore = useAccountStore()
const positionStore = usePositionStore()
const derivativeStore = useDerivativeStore()
const { $onError } = useNuxtApp()
const { fetchTokenUsdPrice } = useToken()

const FilterList = {
  Balances: 'balances',
  Positions: 'positions'
}

const hideBalances = ref(false)
const activeType = ref(FilterList.Balances)
const status = reactive(new Status(StatusType.Loading))
const usdPriceStatus = reactive(new Status(StatusType.Loading))

const usdcConvertMarket = ref<UiSpotMarketWithToken | undefined>(undefined)

const { accountBalancesWithToken: currentSubaccountBalances } = useBalance()

onMounted(() => {
  handleViewFromRoute()
  initBalances()

  useEventBus(BusEvents.FundingRefresh).on(refreshBalances)
  useEventBus<Token>(BusEvents.ConvertUsdc).on(setUsdcConvertMarket)
})

onBeforeUnmount(() => {
  modalStore.closeModal(Modal.AssetDetails)
  spotStore.reset()
})

function setUsdcConvertMarket(token: Token) {
  usdcConvertMarket.value = spotStore.usdcConversionModalMarkets.find(
    (market) =>
      market.baseToken.symbol === token.symbol &&
      market.quoteToken.symbol === USDCSymbol.WormholeEthereum
  )
}

function initBalances() {
  handleViewFromRoute()

  Promise.all([
    tokenStore.fetchBitcoinUsdPrice(),
    spotStore.fetchUsdcConversionMarkets(),
    derivativeStore.streamSubaccountOrders(),
    positionStore.fetchSubaccountPositions()
  ])
    .catch($onError)
    .finally(() => {
      status.setIdle()

      refreshUsdTokenPrice()
    })

  Promise.all([
    derivativeStore.streamSubaccountOrders(),
    derivativeStore.streamMarketsMarkPrices(),
    positionStore.streamSubaccountPositions()
  ])
}

function handleViewFromRoute() {
  const filterListValues = Object.values(FilterList)
  const tab = filterListValues.find((tab) => tab === route.query.view)

  if (tab) {
    activeType.value = tab
  }
}

function refreshBalances() {
  Promise.all([derivativeStore.fetchSubaccountOrders()])
}

function refreshUsdTokenPrice() {
  fetchTokenUsdPrice(currentSubaccountBalances.value.map((b) => b.token))
    .catch($onError)
    .finally(() => usdPriceStatus.setIdle())
}

function handleHideBalances(value: boolean) {
  hideBalances.value = value
}

watch(
  () => accountStore.subaccountId,
  () => positionStore.fetchSubaccountPositions()
)

useIntervalFn(refreshUsdTokenPrice, 1000 * 30)
</script>

<template>
  <div class="pt-6 sm:pb-8">
    <h2 class="text-2xl text-qwerty-white font-bold mb-4">
      {{ $t('account.accountOverview') }}
    </h2>

    <span class="text-qwerty-white text-lg mb-1">
      {{ $t('account.netWorth') }}
    </span>

    <PartialsAccountOverview
      :is-loading="status.isLoading() || usdPriceStatus.isLoading()"
      v-bind="{ hideBalances, currentSubaccountBalances }"
      @update:hide-balances="handleHideBalances"
    >
    </PartialsAccountOverview>

    <div class="h-full-flex">
      <CommonTabMenu>
        <AppSelectButton
          v-for="filterType in Object.values(FilterList)"
          :key="`account-tabs-${filterType}`"
          v-model="activeType"
          :value="filterType"
        >
          <template #default="{ active }">
            <NuxtLink
              :to="{
                name: 'account',
                query: { view: filterType }
              }"
            >
              <CommonTabMenuItem :active="active">
                <p v-if="filterType === FilterList.Balances">
                  {{ $t('account.tabs.balances') }}
                </p>

                <p v-if="filterType === FilterList.Positions">
                  {{ $t('account.tabs.positions') }}
                </p>
              </CommonTabMenuItem>
            </NuxtLink>
          </template>
        </AppSelectButton>
      </CommonTabMenu>

      <CommonCard lg class="mt-16 p-4">
        <AppHocLoading :status="status">
          <PartialsAccountBalances
            v-if="activeType === FilterList.Balances"
            v-bind="{
              balances: currentSubaccountBalances,
              hideBalances,
              usdPriceStatus
            }"
          />

          <PartialsAccountPositions
            v-if="activeType === FilterList.Positions"
            v-bind="{ hideBalances, balances: currentSubaccountBalances }"
          />
        </AppHocLoading>
      </CommonCard>
    </div>
    <PartialsAccountBalancesAssetDetails
      v-if="modalStore.modals[Modal.AssetDetails]"
    />
    <PartialsAccountBridge />

    <ModalsAddMargin />
    <ModalsConvertUsdc
      v-if="usdcConvertMarket"
      :balances="currentSubaccountBalances"
      :market="usdcConvertMarket"
    />
  </div>
</template>
