<script lang="ts" setup>
import { BigNumberInBase, BigNumberInWei } from '@injectivelabs/utils'
import { cosmosSdkDecToBigNumber } from '@injectivelabs/sdk-ts'
import { ZERO_IN_BASE } from '@injectivelabs/sdk-ui-ts'
import type { Token } from '@injectivelabs/token-metadata'
import {
  UI_DEFAULT_DISPLAY_DECIMALS,
  HIDDEN_BALANCE_DISPLAY,
  UI_MINIMAL_ABBREVIATION_FLOOR
} from '@/app/utils/constants'
import { AccountBalance, BridgeBusEvents, Modal } from '@/types'

const tokenStore = useTokenStore()
const accountStore = useAccountStore()
const exchangeStore = useExchangeStore()

const props = defineProps({
  isLoading: Boolean,
  hideBalances: Boolean
})

const emit = defineEmits<{
  (e: 'update:hide-balances', state: boolean): void
}>()

const { aggregatedPortfolioBalances } = useBalance()

const aggregatedAccountBalances = computed(() =>
  Object.keys(aggregatedPortfolioBalances.value).reduce(
    (balances, subaccountId) => [
      ...balances,
      ...aggregatedPortfolioBalances.value[subaccountId]
    ],
    [] as AccountBalance[]
  )
)

const stakedAmount = computed(() => {
  if (
    !exchangeStore.feeDiscountAccountInfo ||
    !exchangeStore.feeDiscountAccountInfo.accountInfo
  ) {
    return ZERO_IN_BASE
  }

  return new BigNumberInBase(
    cosmosSdkDecToBigNumber(
      exchangeStore.feeDiscountAccountInfo.accountInfo.stakedAmount
    )
  )
})

const stakedAmountInUsd = computed(() =>
  stakedAmount.value.times(tokenStore.injUsdPrice)
)

const accountTotalBalanceInUsd = computed(() =>
  aggregatedAccountBalances.value
    .reduce(
      (total, balance) =>
        total.plus(
          new BigNumberInWei(balance.accountTotalBalanceInUsd).toBase(
            balance.token.decimals
          )
        ),
      ZERO_IN_BASE
    )
    .plus(stakedAmountInUsd.value)
)

const shouldAbbreviateTotalBalance = computed(() =>
  accountTotalBalanceInUsd.value.gte(UI_MINIMAL_ABBREVIATION_FLOOR)
)

const accountTotalBalanceInBtc = computed(() => {
  if (!tokenStore.btcUsdPrice) {
    return ZERO_IN_BASE
  }

  return accountTotalBalanceInUsd.value.dividedBy(
    new BigNumberInBase(tokenStore.btcUsdPrice)
  )
})

const accountTotalBalanceInBtcToString = computed(() => {
  if (accountTotalBalanceInBtc.value.eq('0')) {
    return '0.00'
  }

  if (accountTotalBalanceInBtc.value.lte('0.0001')) {
    return '< 0.0001'
  }

  return accountTotalBalanceInBtc.value.toFormat(UI_DEFAULT_DISPLAY_DECIMALS)
})

const { valueToString: abbreviatedTotalBalanceToString } =
  useBigNumberFormatter(accountTotalBalanceInUsd, {
    decimalPlaces: shouldAbbreviateTotalBalance.value ? 0 : 2,
    abbreviationFloor: shouldAbbreviateTotalBalance.value
      ? UI_MINIMAL_ABBREVIATION_FLOOR
      : undefined
  })

function toggleHideBalances() {
  emit('update:hide-balances', !props.hideBalances)
}

const modalStore = useModalStore()
function handleDepositClick() {
  // useEventBus<Token | undefined>(BridgeBusEvents.Deposit).emit()
  modalStore.openModal({
    type: Modal.DepositFiat,
    data: { type: 'deposit' }
  })
}

function handleWithdrawClick() {
  // useEventBus<Token | undefined>(BridgeBusEvents.Withdraw).emit()
  modalStore.openModal({
    type: Modal.DepositFiat,
    data: { type: 'withdraw' }
  })
}
</script>

<template>
  <div :class="{ 'mb-8': !isLoading, 'my-4': isLoading }">
    <div
      class="flex justify-between md:items-center gap-4 flex-col md:flex-row"
    >
      <AppSpinner v-if="isLoading" lg />
      <div v-else class="flex items-center justify-start gap-2">
        <span
          v-if="!hideBalances"
          class="text-qwerty-white font-bold text-2xl md:text-3xl"
        >
          &dollar;
          <span class="font-mono">{{ abbreviatedTotalBalanceToString }}</span>
          USD
        </span>
        <span v-else class="text-qwerty-white font-bold text-2xl md:text-3xl">
          &dollar; {{ HIDDEN_BALANCE_DISPLAY }} USD
        </span>

        <span v-if="!hideBalances" class="text-qwerty-white md:text-lg">
          &thickapprox;
          <span class="font-mono">{{ accountTotalBalanceInBtcToString }}</span>
          BTC
        </span>
        <span v-else class="text-qwerty-white md:text-lg">
          &thickapprox; {{ HIDDEN_BALANCE_DISPLAY }} BTC
        </span>

        <div @click="toggleHideBalances">
          <BaseIcon
            v-if="hideBalances"
            name="hide"
            class="w-4 h-4 text-qwerty-white hover:text-qwerty-primary cursor-pointer"
          />

          <BaseIcon
            v-else
            name="show"
            class="w-4 h-4 text-qwerty-white hover:text-qwerty-primary cursor-pointer"
          />
        </div>
      </div>

      <div
        v-if="!isLoading && accountStore.isDefaultSubaccount"
        class="flex items-center justify-between md:justify-end sm:gap-4"
      >
        <AppButton
        class="bg-qwerty-primary text-qwerty-background"
        @click="handleDepositClick"
      >
          <span class="font-semibold">
            {{ $t('account.depositFiat') }}
          </span>
        </AppButton>

        <AppButton
        class="bg-qwerty-primary text-qwerty-background"
        @click="handleWithdrawClick"
      >
          <span class="font-semibold">
            {{ $t('account.withdrawFiat') }}
          </span>
        </AppButton>
      </div>
    </div>

    <PartialsAccountSubaccountSelector
      v-if="!isLoading && accountStore.hasMultipleSubaccounts"
      v-bind="{
        hideBalances
      }"
    />
  </div>
</template>
