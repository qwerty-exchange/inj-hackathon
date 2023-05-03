<script lang="ts" setup>
import { PropType } from 'vue'
import { FundingPayment } from '@injectivelabs/sdk-ts'
import { UI_DEFAULT_MAX_DISPLAY_DECIMALS } from '@/app/utils/constants'
import { getMarketRoute } from '@/app/utils/market'

const props = defineProps({
  fundingPayment: {
    required: true,
    type: Object as PropType<FundingPayment>
  }
})

const { market, time, total, minimalDisplayAmount } = useFundingPayment(
  computed(() => props.fundingPayment)
)

const marketRoute = computed(() => {
  if (!market.value) {
    return undefined
  }

  return getMarketRoute(market.value)
})
</script>

<template>
  <CommonTableRow v-if="market" dense>
    <div
      class="flex items-center justify-between col-span-2 text-xs leading-5 pb-1"
    >
      <NuxtLink :to="marketRoute" class="flex flex-col cursor-pointer">
        <div class="flex items-center justify-start">
          <div v-if="market.baseToken" class="w-4 h-4">
            <CommonTokenIcon :token="market.baseToken" sm />
          </div>
          <div class="ml-1">
            <span class="text-qwerty-white font-semibold text-xs">
              {{ market.ticker }}
            </span>
          </div>
        </div>
        <span class="text-qwerty-white text-xs font-mono">{{ time }}</span>
      </NuxtLink>

      <div>
        <AppNumber
          v-if="total.abs().gt(minimalDisplayAmount)"
          data-cy="funding-payments-total-table-data"
          :class="{
            'text-qwerty-green': total.gte(0),
            'text-qwerty-red': total.lt(0)
          }"
          :decimals="UI_DEFAULT_MAX_DISPLAY_DECIMALS"
          :prefix="total.lt(0) ? '-' : ''"
          :number="total"
        >
          <template #addon>
            <span class="text-2xs text-qwerty-white">
              {{ market.quoteToken.symbol }}
            </span>
          </template>
        </AppNumber>
        <span
          v-else
          :class="{
            'text-qwerty-green': total.gte(0),
            'text-qwerty-red': total.lt(0)
          }"
        >
          {{ `< ${minimalDisplayAmount.toFormat(6)}` }}
        </span>
      </div>
    </div>
  </CommonTableRow>
</template>
