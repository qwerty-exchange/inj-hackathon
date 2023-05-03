<script lang="ts" setup>
import { MarketType } from '@injectivelabs/sdk-ui-ts'
import { DefaultMarket, TradeClickOrigin } from '@/types'
import { amplitudeTracker } from '@/app/providers/AmplitudeTracker'
import {
  getDefaultPerpetualMarketRouteParams,
  getDefaultSpotMarketRouteParams
} from '@/app/utils/market'

const walletStore = useWalletStore()
const route = useRoute()

const defaultPerpetualMarketRoute = getDefaultPerpetualMarketRouteParams()
const defaultSpotMarketRoute = getDefaultSpotMarketRouteParams()

const tradeDropdownShown = ref(false)
const rewardsDropdownShown = ref(false)

const hasTradeActive = computed(() => {
  return (
    route.fullPath.startsWith('/spot/') ||
    route.fullPath.startsWith('futures') ||
    route.name === 'convert'
  )
})

function handleSpotTradeClickedTrack() {
  amplitudeTracker.submitTradeClickedTrackEvent({
    market: DefaultMarket.Spot,
    marketType: MarketType.Spot,
    origin: TradeClickOrigin.TopMenu
  })
}

function handlePerpetualTradeClickedTrack() {
  amplitudeTracker.submitTradeClickedTrackEvent({
    market: DefaultMarket.Perpetual,
    marketType: MarketType.Perpetual,
    origin: TradeClickOrigin.TopMenu
  })
}

function handleTradeDropdownShownChange(value: boolean) {
  tradeDropdownShown.value = value

  if (value) {
    rewardsDropdownShown.value = false
  }
}
</script>

<template>
  <div>
    <nav class="block flex-1 lg:flex">
      <LayoutNavItem :to="{ name: 'index' }" class="block lg:hidden">
        {{ $t('navigation.home') }}
      </LayoutNavItem>
      <LayoutNavItem
        :to="{ name: 'markets' }"
        class="block"
        data-cy="header-markets-link"
      >
        {{ $t('trade.markets') }}
      </LayoutNavItem>

      <LayoutNavHoverMenu
        :shown="tradeDropdownShown"
        @dropdown:toggle="handleTradeDropdownShownChange"
      >
        <template #default>
          <LayoutNavItemDummy
            id="trade-dropdown"
            class="hidden lg:block"
            :class="{ 'router-link-exact-active': hasTradeActive }"
          >
            {{ $t('navigation.trade') }}
          </LayoutNavItemDummy>
        </template>

        <template #content>
          <NuxtLink
            :to="defaultSpotMarketRoute"
            class="p-4 block rounded-t group hover:bg-qwerty-shade3 relative z-50 bg-qwerty-shade2"
            data-cy="header-trade-link"
            @click="handleSpotTradeClickedTrack"
          >
            <p
              class="font-semibold text-base group-hover:text-qwerty-primary text-qwerty-white"
            >
              {{ $t('navigation.spot') }}
            </p>
            <p class="text-sm text-qwerty-secondary2 mt-1">
              {{ $t('navigation.spotDescription') }}
            </p>
          </NuxtLink>
          <div class="border-t border-qwerty-shade3"></div>
          <NuxtLink
            :to="defaultPerpetualMarketRoute"
            class="p-4 block group hover:bg-qwerty-shade3 relative z-50 bg-qwerty-shade2"
            data-cy="header-trade-link"
            @click="handlePerpetualTradeClickedTrack"
          >
            <p
              class="font-semibold text-base text-qwerty-white group-hover:text-qwerty-primary"
            >
              {{ $t('navigation.perpetual') }}
            </p>
            <p class="text-sm text-qwerty-secondary2 mt-1">
              {{ $t('navigation.perpetualDescription') }}
            </p>
          </NuxtLink>
          <div class="border-t border-qwerty-shade3"></div>
          <NuxtLink
            :to="{ name: 'convert' }"
            class="p-4 block rounded-b group hover:bg-qwerty-shade3 relative z-50 bg-qwerty-shade2"
            data-cy="header-convert-link"
          >
            <p
              class="font-semibold text-base text-qwerty-white group-hover:text-qwerty-primary"
            >
              {{ $t('navigation.convert') }}
            </p>
            <p class="text-sm text-qwerty-secondary2 mt-1">
              {{ $t('navigation.convertDescription') }}
            </p>
          </NuxtLink>
        </template>
      </LayoutNavHoverMenu>

      <LayoutNavItem to="/pawnshop" class="block" data-cy="header-markets-link">
        Pawnshop
      </LayoutNavItem>

      <LayoutNavMobile />

      <!-- <LayoutNavItem
        :to="{ name: 'trade-and-earn' }"
        class="block"
        data-cy="header-rewards-link"
      >
        {{ $t('navigation.rewards') }}
      </LayoutNavItem> -->

      <!-- <LayoutNavItem
        class="block"
        data-cy="nav-leaderboard-link"
        :to="{ name: 'leaderboard' }"
      >
        {{ $t('navigation.leaderboard') }}
      </LayoutNavItem> -->

      <LayoutNavItem
        v-if="walletStore.isUserWalletConnected"
        class="block lg:hidden"
        data-cy="header-account-link"
        :to="{ name: 'account', query: { view: 'balances' } }"
      >
        {{ $t('navigation.account') }}
      </LayoutNavItem>

      <LayoutNavItem
        v-if="walletStore.isUserWalletConnected"
        class="block lg:hidden"
        data-cy="nav-activity-link"
        :to="{ name: 'activity' }"
      >
        {{ $t('navigation.activity') }}
      </LayoutNavItem>
    </nav>
  </div>
</template>
