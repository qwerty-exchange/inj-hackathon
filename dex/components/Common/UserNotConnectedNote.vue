<script lang="ts" setup>
import { Modal } from '@/types'

const modalStore = useModalStore()

defineProps({
  cta: Boolean
})

function handleConnect() {
  modalStore.openModal({ type: Modal.Connect })
}
</script>

<template>
  <div
    class="flex h-full w-full items-center justify-center text-center"
    data-cy="wallet-not-connected-modal-warning"
  >
    <div v-if="cta" class="text-center py-4">
      <p class="text-xs text-qwerty-white w-full">
        {{ $t('connect.pleaseConnectToYourWallet') }}
      </p>
      <AppButton
        class="bg-qwerty-primary text-qwerty-background font-semibold mt-2"
        data-cy="wallet-not-connected-warning-modal-connect-button"
        @click="handleConnect"
      >
        {{ $t('connect.connect') }}
      </AppButton>
    </div>
    <p v-else class="text-xs text-qwerty-white py-4">
      {{ $t('connect.pleaseConnectToYourWalletCta') }}
    </p>
  </div>
</template>
