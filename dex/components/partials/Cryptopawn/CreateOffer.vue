<script lang="ts" setup>
import { Status, StatusType } from '@injectivelabs/utils'
import { useCryptoPawnStore } from '~~/store/cryptopawn'
import { PropositionType } from '~~/types/cryptopawn'
import { Modal } from '@/types'

const pawn = useCryptoPawnStore()
const tokens = useTokenStore()
const balances = useAccountStore()

const modalStore = useModalStore()

const isModalOpen = computed(() => modalStore.modals[Modal.CreateOfferCryptoPawn])
const walletStore = useWalletStore()

const modalData = computed(() => modalStore.data[Modal.CreateOfferCryptoPawn])

const form = reactive({ timeValue: -1 })

const { t } = useLang()

const { success, error } = useNotifications()

const status = reactive(new Status(StatusType.Idle))

const propositionType = computed(() => modalData.value.type)

const state = reactive({
  funds: {
    assets: {
      amount: '',
      denom: 'inj'
    },
    deposit: {
      amount: '',
      denom: 'inj'
    },
    premium: {
      amount: '',
      denom: 'inj'
    }
  },
  expiry: 0,
  period: 0
})

const configModal = computed(() => {
  if (propositionType.value === PropositionType.Ask) {
    return {
      title: 'Get a loan',
      assets: 'Assets to borrow',
      assetsShowUserBalance: false,
      deposit: 'Collateral deposit',
      depositShowUserBalance: true,
      premium: 'Premium for lender',
      premiumShowUserBalance: true,
      period: 'Duration of the loan',
      expiry: 'Offer expiry date',
      actionButton: 'Get a loan'
    }
  }
  return {
    title: 'Give a loan',
    assets: 'Assets to lend',
    assetsShowUserBalance: true,
    deposit: 'Collateral deposit',
    depositShowUserBalance: false,
    premium: 'Premium for you',
    premiumShowUserBalance: false,
    period: 'Duration of the loan',
    expiry: 'Offer expiry date',
    actionButton: 'Give a loan'
  }
})
const validateForm = () => {
  Object.entries(state.funds).map(([_type, token]) => {
    if (token.amount === '') {
      return true
    }
  })

  if (state.expiry <= 0 || state.period <= 0) {
    return true
  }

  return false
}

const hasFormErrors = ref(validateForm())

watch(state, () => {
  hasFormErrors.value = validateForm()
})

const handleChangeExpiration = (expiry: string) => {
  state.expiry = parseInt(expiry)
}

const handleChangePeriod = (period: string) => {
  state.period = parseInt(period)
}

const handleChangeDepositAmount = ({ amount }: { amount: string }) => {
  state.funds.deposit.amount = amount
}
const handleChangeDepositDenom = (denom: string) => {
  state.funds.deposit.denom = denom
}

const handleChangeAssetsAmount = ({ amount }: { amount: string }) => {
  state.funds.assets.amount = amount
}

const handleChangeAssetsDenom = (denom: string) => {
  state.funds.assets.denom = denom
}

const handleChangePremiaAmount = ({ amount }: { amount: string }) => {
  state.funds.premium.amount = amount
}

const handleChangePremiaDenom = (denom: string) => {
  state.funds.premium.denom = denom
}

const closeModal = () => {
  modalStore.closeModal(Modal.CreateOfferCryptoPawn)
}

const handleCreateProposition = async () => {
  const { funds, expiry, period } = state

  const transformedMsg = {
    ...funds,
    proposition_type: propositionType.value,
    expiry,
    period
  }

  status.setLoading()

  await pawn
    .createProposition(transformedMsg)
    .then(() => {
      closeModal()
      success({ title: 'success' })
      modalStore.openModal({ type: Modal.Notify })
      // openNotifyModal()
      return pawn.fetchPropositions()
    })
    .catch((e) => {
      console.log(e)
      error({ title: 'Error' })
    })
    .finally(() => {
      status.setIdle()
    })
}
</script>

<template>
  <AppModal :show="isModalOpen" is-always-open sm @modal:closed="closeModal">
    <template #title>
      <h3 class="text-base">{{ configModal.title }}</h3>
    </template>
    <AppHocLoading :status="status">
      <div class="mb-4" />
      <AppSelectToken
        v-bind="{
          denom: state.funds.assets.denom,
          hideMax: true,
          required: true,
          hideBalance: !configModal.assetsShowUserBalance,
          options: tokens.tradeableTokens.map((item) => ({
            token: item,
            balance: balances.balanceMap[item.denom],
            denom: item.denom
          }))
        }"
        @update:denom="handleChangeAssetsDenom"
        @update:amount="handleChangeAssetsAmount"
      >
        <span>{{ configModal.assets }}</span>
      </AppSelectToken>
      <div class="mb-4" />
      <AppSelectToken
        v-bind="{
          denom: state.funds.deposit.denom,
          hideMax: true,
          required: true,
          hideBalance: !configModal.depositShowUserBalance,
          options: tokens.tradeableTokens.map((item) => ({
            token: item,
            balance: balances.balanceMap[item.denom],
            denom: item.denom
          }))
        }"
        @update:denom="handleChangeDepositDenom"
        @update:amount="handleChangeDepositAmount"
      >
        <span>{{ configModal.deposit }}</span>
      </AppSelectToken>
      <div class="mb-4" />
      <AppSelectToken
        v-model:denom="state.funds.premium.denom"
        v-bind="{
          hideMax: true,
          required: true,
          hideBalance: !configModal.premiumShowUserBalance,
          options: tokens.tradeableTokens.map((item) => ({
            token: item,
            balance: balances.balanceMap[item.denom],
            denom: item.denom
          }))
        }"
        @update:denom="handleChangePremiaDenom"
        @update:amount="handleChangePremiaAmount"
      >
        <span>{{ configModal.premium }}</span>
      </AppSelectToken>
      <div class="mb-4" />
      <div class="grid grid-cols-2 gap-4">
        <div class="bg-qwerty-background rounded-xl p-4">
          <div class="text-sm font-semibold text-qwerty-white flex items-center justify-between mb-2">
            {{ configModal.period }}
          </div>
          <div class="flex justify-between items-center">
            <AppInputNumeric
              v-model="state.period"
              sm
              no-padding
              transparent-bg
              input-classes="p-0 text-xl font-bold"
              :placeholder="'10'"
              @update:model-value="handleChangePeriod"
            />
            <span class="text-xl font-bold">days</span>
          </div>
        </div>
        <div class="bg-qwerty-background rounded-xl p-4">
          <div class="text-sm font-semibold text-qwerty-white flex items-center justify-between mb-2">
            {{ configModal.expiry }}
          </div>
          <div class="flex justify-between items-center">
            <AppInputNumeric
              v-model="state.expiry"
              sm
              no-padding
              transparent-bg
              input-classes="p-0 text-xl font-bold"
              :placeholder="'60'"
              @update:model-value="handleChangeExpiration"
            />
            <span class="text-xl font-bold">days</span>
          </div>
        </div>
      </div>
      <div class="mb-4" />
      <AppButton :disabled="hasFormErrors" lg class="w-full bg-qwerty-primary text-qwerty-background font-semibold" @click="handleCreateProposition">
        {{ configModal.actionButton }}
      </AppButton>
    </AppHocLoading>
  </AppModal>
</template>
