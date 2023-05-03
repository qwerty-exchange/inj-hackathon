<script setup lang="ts">
import { Modal } from '@/types'
import { Status, StatusType } from '@injectivelabs/utils'

const modalStore = useModalStore()
const pawn = useCryptoPawnStore()

const isModalOpen = computed(() => modalStore.modals[Modal.OfferDetailsCryptoPawn])
const { success, error } = useNotifications()
const modalData = computed(() => modalStore.data[Modal.OfferDetailsCryptoPawn])
const status = reactive(new Status(StatusType.Idle))
const acceptProposition = () => {
  status.setLoading()

  pawn
    .acceptProposition(modalData.value.offer.id)
    .then(() => {
      closeModal()
      success({ title: 'Success' })
      modalStore.openModal({ type: Modal.Notify })
      return pawn.fetchPropositions()
    })
    .catch((err) => {
      console.log(err)
      error({ title: 'Error' })
    })
    .finally(() => {
      status.setIdle()
    })
}

const closeProposition = () => {
  status.setLoading()

  pawn
    .closeProposition(modalData.value.offer.id)
    .then(() => {
      closeModal()
      success({ title: 'Success' })
      return pawn.fetchPropositions()
    })
    .catch(() => {
      error({ title: 'Error' })
    })
    .finally(() => {
      status.setIdle()
    })
}

const type = computed(() => modalData.value.offer.proposition_type)
const action = computed(() => modalData.value.offer.action)

const configModal = computed(() => {
  // ACCEPT ASK
  if (action.value === 'Accept' && type.value === 'bid') {
    return {
      title: 'Get a loan',
      assets: 'Assets to borrow',
      deposit: 'Collateral deposit',
      premium: 'Premium for lender',
      period: 'Duration of the loan',
      expiry: 'Time to repay',
      actionButton: 'Get a loan',
      message: () => {
        return `Once you accept offer, we will collect your deposit and the premium (${modalData.value.offer.deposit.amount} ${modalData.value.offer.deposit.denom} + ${modalData.value.offer.premium.amount} ${modalData.value.offer.premium.denom}) for the lender. You will receive (${modalData.value.offer.assets.amount} ${modalData.value.offer.assets.denom}). The time for repayment is ${modalData.value.offer.period}, in case of non-payment the deposit will be forfeited.`
      }
    }
  }

  // ACCEPT BID
  if (action.value === 'Accept' && type.value === 'ask') {
    return {
      title: 'Give a loan',
      assets: 'Assets to lend',
      deposit: 'Collateral deposit',
      premium: 'Premium for you',
      period: 'Duration of the loan',
      expiry: 'Time to repay',
      actionButton: 'Give a loan',
      message: () => {
        return `Once you accept offer, we will collect your assets (${modalData.value.offer.assets.amount} ${modalData.value.offer.assets.denom}) for the borrower. You will receive a premium (${modalData.value.offer.premium.amount} ${modalData.value.offer.premium.denom}). The time for repayment is ${modalData.value.offer.period}, in case of non-payment you will receive borrower's deposit (${modalData.value.offer.deposit.amount} ${modalData.value.offer.deposit.denom}).`
      }
    }
  }

  // CLOSE ASK
  if (action.value === 'Close' && type.value === 'ask') {
    return {
      title: 'Get a loan',
      assets: 'Assets to borrow',
      deposit: 'Collateral deposit',
      premium: 'Premium for lender',
      period: 'Duration of the loan',
      expiry: 'Time to repay',
      actionButton: 'Close a loan offer',
      message: null
    }
  }

  // CLOSE BID
  if (action.value === 'Close' && type.value === 'bid') {
    return {
      title: 'Get a loan',
      assets: 'Assets to lend',
      deposit: 'Collateral deposit',
      premium: 'Premium for you',
      period: 'Duration of the loan',
      expiry: 'Time to repay',
      actionButton: 'Close a loan offer',
      message: null
    }
  }
})

const closeModal = () => {
  modalStore.closeModal(Modal.OfferDetailsCryptoPawn)
}
</script>

<template>
  <AppModal sm :show="isModalOpen" @modal:closed="closeModal">
    <template #title>
      <h3 class="text-base">{{ configModal?.title }}</h3>
    </template>
    <AppHocLoading :status="status">
      <div class="mb-5">
        <div class="font-bold">{{ configModal?.assets }}</div>
        <div class="mb-4">{{ modalData.offer.assets.amount }} {{ modalData.offer.assets.denom }}</div>
        <div class="mb-4 grid grid-cols-2 gap-4">
          <div>
            <div class="font-bold">{{ configModal?.deposit }}</div>
            <div>{{ modalData.offer.deposit.amount }} {{ modalData.offer.deposit.denom }}</div>
          </div>
          <div>
            <div class="font-bold">{{ configModal?.premium }}</div>
            <div>{{ modalData.offer.premium.amount }} {{ modalData.offer.premium.denom }}</div>
          </div>
        </div>
        <div class="font-bold">{{ configModal?.period }}</div>
        <div>{{modalData.offer.period}}</div>
      </div>
      <div v-if="configModal?.message !== null" class="mb-6 text-qwerty-secondary2 text-sm bg-qwerty-background p-3 rounded-xl">
        {{ configModal?.message && configModal?.message() }}
      </div>
      <AppButton v-if="modalData.offer.action === 'Accept'" @click="acceptProposition" class="w-full bg-qwerty-primary text-black">{{
        configModal?.actionButton
      }}</AppButton>
      <AppButton v-else @click="closeProposition" class="w-full bg-qwerty-primary text-black">{{ configModal?.actionButton }}</AppButton>
    </AppHocLoading>
  </AppModal>
</template>
