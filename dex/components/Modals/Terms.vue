<script lang="ts" setup>
import { Modal } from '@/types'

const modalStore = useModalStore()

const isModalOpen = computed(() => modalStore.modals[Modal.Terms])

function handleConfirm() {
  closeModal()

  modalStore.openModal({ type: Modal.Connect })
}

function handleCancel() {
  closeModal()
}

function closeModal() {
  modalStore.closeModal(Modal.Terms)
}
</script>

<template>
  <AppModal :show="isModalOpen" @modal:closed="closeModal">
    <template #title>
      <h3>
        {{ $t('Acknowledge Terms') }}
      </h3>
    </template>

    <div class="relative">
      <i18n-t keypath="terms.disclaimer_note" tag="p" class="text-sm">
        <template #terms>
          <NuxtLink
            target="_blank"
            class="text-qwerty-primary hover:text-opacity-80"
            to="https://qwerty.exchange/terms-and-conditions.pdf"
          >
            {{ $t('terms.termsAndCondition') }}
          </NuxtLink>
        </template>

        <template #policy>
          <NuxtLink
            target="_blank"
            class="text-qwerty-primary hover:text-opacity-80"
            to="https://qwerty.exchange/privacy-policy.pdf"
          >
            {{ $t('terms.privacyPolicy') }}
          </NuxtLink>
        </template>

        <template #disclaimer>
          <NuxtLink
            target="_blank"
            class="text-qwerty-primary hover:text-opacity-80"
            to="https://qwerty.exchange/disclaimer.pdf"
          >
            {{ $t('terms.disclaimer') }}
          </NuxtLink>
        </template>
      </i18n-t>

      <ul
        class="p-4 bg-qwerty-shade2 mt-6 text-xs text-qwerty-white rounded-lg border border-qwerty-shade3"
      >
        <li class="font-bold text-qwerty-white">
          {{ $t('terms.title') }}
        </li>
        <li class="mt-2">
          {{ $t('terms.acknowledge_1') }}
        </li>
        <li class="mt-2">
          {{ $t('terms.acknowledge_2') }}
        </li>
        <li class="mt-2">
          {{ $t('terms.acknowledge_3') }}
        </li>
        <li class="mt-2">
          {{ $t('terms.acknowledge_4') }}
        </li>
        <li class="mt-2">
          {{ $t('terms.acknowledge_5') }}
        </li>
      </ul>
      <div class="mt-6 flex items-center justify-center gap-3">
        <AppButton
          class="bg-qwerty-primary text-qwerty-background font-semibold"
          @click="handleConfirm"
        >
          {{ $t('common.confirm') }}
        </AppButton>
        <AppButton
          class="text-qwerty-white bg-qwerty-red font-semibold hover:text-qwerty-primary"
          @click="handleCancel"
        >
          {{ $t('common.cancel') }}
        </AppButton>
      </div>
    </div>
  </AppModal>
</template>
