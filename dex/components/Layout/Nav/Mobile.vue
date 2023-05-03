<script lang="ts" setup>
import {
  getDefaultPerpetualMarketRouteParams,
  getDefaultSpotMarketRouteParams
} from '@/app/utils/market'

const tradeMenuOpen = ref(false)
const rewardsMenuOpen = ref(false)

const defaultPerpetualMarketRoute = getDefaultPerpetualMarketRouteParams()
const defaultSpotMarketRoute = getDefaultSpotMarketRouteParams()

function handleToggleTradeMenu() {
  tradeMenuOpen.value = !tradeMenuOpen.value
}

function handleToggleRewardsMenu() {
  rewardsMenuOpen.value = !rewardsMenuOpen.value
}
</script>

<template>
  <div class="block lg:hidden">
    <AppAccordion
      :is-open="tradeMenuOpen"
      sm
      @panel:toggle="handleToggleTradeMenu"
    >
      <template #title>
        <div class="text-sm font-semibold">
          {{ $t('navigation.trade') }}
        </div>
      </template>

      <template #content>
        <LayoutNavItem :to="defaultSpotMarketRoute">
          <span class="font-normal tracking-wide">{{
            $t('navigation.spot')
          }}</span>
        </LayoutNavItem>

        <LayoutNavItem :to="defaultPerpetualMarketRoute">
          <span class="font-normal tracking-wide">{{
            $t('navigation.perpetual')
          }}</span>
        </LayoutNavItem>

        <LayoutNavItem :to="{ name: 'convert' }">
          <span class="font-normal tracking-wide">{{
            $t('navigation.convert')
          }}</span>
        </LayoutNavItem>
      </template>
    </AppAccordion>

  </div>
</template>
