<script lang="ts" setup>
import { Modal } from '@/types'

const appStore = useAppStore()
const walletStore = useWalletStore()
const ninjaPassStore = useNinjaPassStore()
const modalStore = useModalStore()
const confetti = useConfetti()

const isModalOpen = computed(() => modalStore.modals[Modal.NinjaPassWinner])

const ninjaPassCode = computed(() => {
  if (!ninjaPassStore.codes) {
    return
  }

  return ninjaPassStore.codes[0]
})

const ninjaPassUrl = computed(() => {
  if (!ninjaPassCode.value) {
    return
  }

  return `https://ninjapass.injective.com/?code=${ninjaPassCode.value.code}&address=${walletStore.injectiveAddress}`
})

watch(
  () => ninjaPassCode,
  (code) => {
    if (code && !appStore.userState.ninjaPassWinnerModalViewed) {
      modalStore.openModal({ type: Modal.NinjaPassWinner })

      confetti.showConfetti()
    }
  }
)

function closeModal() {
  modalStore.closeModal(Modal.NinjaPassWinner)

  appStore.setUserState({
    ...appStore.userState,
    ninjaPassWinnerModalViewed: true
  })
}
</script>

<template>
  <AppModal :show="isModalOpen" sm hide-close-button @modal:closed="closeModal">
    <template #title>
      <h3 class="normal-case">
        {{ $t('ninjaPass.congratulations') }}
      </h3>
    </template>

    <div class="flex flex-col">
      <span class="text-sm mb-4">
        {{ $t('ninjaPass.title') }}
      </span>
      <span class="text-sm">
        {{ $t('ninjaPass.description') }}
      </span>
      <div class="grid grid-cols-1 gap-4 mt-6 sm:grid-cols-2">
        <a
          :href="ninjaPassUrl"
          target="_blank"
          class="bg-qwerty-primary text-qwerty-background py-2 h-10 rounded border flex items-center justify-center gap-2"
        >
          <span class="font-semibold text-qwerty-white">
            {{ $t('ninjaPass.verifyNow') }}
          </span>
          <BaseIcon name="external-link" class="w-4 h-4 text-qwerty-white" />
        </a>

        <button
          class="bg-transparent py-2 h-10 rounded border border-blue-100"
          @click="closeModal"
        >
          <span class="font-semibold text-qwerty-white">
            {{ $t('ninjaPass.later') }}
          </span>
        </button>
      </div>
    </div>
  </AppModal>
</template>
