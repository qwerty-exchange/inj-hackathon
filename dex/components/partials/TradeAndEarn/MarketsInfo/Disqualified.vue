<script lang="ts" setup>
const spotStore = useSpotStore()
const derivativeStore = useDerivativeStore()

const disqualifiedMarkets = computed(() =>
  [...derivativeStore.markets, ...spotStore.markets]
    .filter((m) =>
      useTradeReward().disqualifiedMarketIdsList.value.includes(m.marketId)
    )
    .map((m) => m.ticker)
)
</script>

<template>
  <PartialsCommonStatsItem>
    <template #value>
      <span v-if="disqualifiedMarkets.length > 0" class="font-mono text-lg">
        {{ disqualifiedMarkets.join(', ') }}
      </span>
      <span v-else class="text-xs font-normal">
        {{ $t('trade.there_are_no_disqualified_markets_on_this_relayer') }}
      </span>
    </template>
    <template #title>
      <div class="flex items-center justify-center text-qwerty-white">
        {{ $t('trade.disqualified_markets') }}
        <CommonInfoTooltip
          class="ml-2 text-qwerty-white"
          :tooltip="$t('trade.disqualified_markets_tooltip')"
        />
      </div>
    </template>
  </PartialsCommonStatsItem>
</template>
