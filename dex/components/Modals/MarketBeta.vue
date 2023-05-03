<script lang="ts" setup>
import { Modal } from '@/types'

const modalStore = useModalStore()

const isModalOpen = computed(() => modalStore.modals[Modal.MarketBeta])

function closeModal() {
  modalStore.closeModal(Modal.MarketBeta)
}
</script>

<template>
  <AppModal :show="isModalOpen" sm hide-close-button>
    <template #title>
      <h3>
        {{ $t('marketBeta.title') }}
      </h3>
    </template>

    <div class="relative">
      <p
        class="text-center text-sm text-qwerty-white"
        v-text="$t('marketBeta.description')"
      ></p>

      <div class="mt-6 flex items-center justify-center">
        <AppButton
          class="bg-qwerty-primary text-qwerty-background"
          @click="closeModal"
        >
          {{ $t('marketBeta.I Understand') }}
        </AppButton>
      </div>
    </div>
  </AppModal>
</template>
