<script lang="ts" setup>
import { Status, StatusType } from '@injectivelabs/utils'
import { Modal } from '@/types'

const modalStore = useModalStore()
const walletStore = useWalletStore()

const isModalOpen = computed(() => modalStore.modals[Modal.DepositFiat])

const modalData = computed(() => modalStore.data[Modal.DepositFiat])
const status = reactive(new Status(StatusType.Loading))

function closedModal() {
  modalStore.closeModal(Modal.DepositFiat)
  status.setLoading()
}
</script>

<template>
  <AppModal
    md
    class="deposit-fiat-modal deposit-fiat-modal-kado"
    :show="isModalOpen"
    :is-always-open="true"
    @modal:closed="closedModal"
  >
    <template #title>
      <h3>
        {{ $t('Bridge fiat') }}
      </h3>
    </template>
    <div class="modal-deposit-fiat relative">
      <div class="flex justify-center">
        <AppSpinner v-if="status.isLoading()" lg />
        <iframe
          title="Buy & Sell Crypto"
          :src="`https://app.kado.money?apiKey=eb0698f2-fe4a-4760-a203-9c05daae6c71&onRevCurrency=USDT&cryptoList=USDT&networkList=injective&network=injective&onToAddress=${
            walletStore.injectiveAddress
          }&product=${modalData.type === 'deposit' ? 'buy' : 'sell'}`"
          style="height: 800px; width: 680px"
          :style="{ display: status.isCompleted() ? 'block' : 'none' }"
          @load="status.setCompleted()"
        />
      </div>
    </div>
  </AppModal>
</template>

<style>
div.deposit-fiat-modal label {
  @apply text-base;
}

.deposit-fiat-modal div[aria-labelledby='modal-headline'] {
  max-width: 890px;
}

.deposit-fiat-modal-kado div[aria-labelledby='modal-headline'] {
  width: 740px;
}
</style>
