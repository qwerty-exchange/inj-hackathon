<script lang="ts" setup>
import { PropType } from 'vue'
import { OrderSide } from '@injectivelabs/ts-types'
import { TradeField, TradeForm, UiMarketWithToken } from '@/types'

const formValues = useFormValues<TradeForm>()

defineProps({
  market: {
    type: Object as PropType<UiMarketWithToken>,
    required: true
  }
})

const filterList = [OrderSide.Buy, OrderSide.Sell]

const { value: orderSide } = useStringField({
  name: TradeField.OrderSide,
  initialValue: OrderSide.Buy,
  rule: 'required'
})

function handleSelectOrderSide() {
  formValues.value[TradeField.ProportionalPercentage] = 0
}
</script>

<template>
  <div class="rounded flex h-9">
    <AppSelectButton
      v-for="orderSideItem in filterList"
      :key="`trade-form-order-${orderSideItem}`"
      v-model="orderSide"
      class="w-1/2 bg-gray-1000 shadow-sm"
      :value="orderSideItem"
      @update:modelValue="handleSelectOrderSide"
    >
      <template #default="{ active }">
        <span
          class="uppercase rounded text-xs tracking-wide border px-5 h-9 flex"
          :class="{
            'text-qwerty-white border-transparent': !active,
            'bg-qwerty-green border-qwerty-shade3':
              active && orderSide === OrderSide.Buy,
            'bg-qwerty-red border-qwerty-shade3':
              active && orderSide === OrderSide.Sell
          }"
        >
          <span v-if="orderSideItem === OrderSide.Buy" class="m-auto">
            {{
              $t('trade.buy_asset', {
                asset: market.baseToken.symbol
              })
            }}
          </span>
          <span v-else class="m-auto">
            {{
              $t('trade.sell_asset', {
                asset: market.baseToken.symbol
              })
            }}
          </span>
        </span>
      </template>
    </AppSelectButton>
  </div>
</template>
