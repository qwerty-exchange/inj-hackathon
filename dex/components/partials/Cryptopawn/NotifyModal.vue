<script lang="ts" setup>
import {
  newEvmClient,
  newEvmConfig
} from '@notifi-network/notifi-frontend-client'
import { walletStrategy } from '~~/app/wallet-strategy'
import { Modal } from '~~/types'

const { success } = useNotifications()

const emit = defineEmits<{
  (e: 'modal:close'): void
}>()

const modalStore = useModalStore()

const onClose = () => {
  modalStore.closeModal(Modal.Notify)
  emit('modal:close')
}

const isModalOpen = computed(() => modalStore.modals[Modal.Notify])

const wallet = useWalletStore()
const accountAddress = wallet.address
const tenantId = 'qwertyexchange'
const blockchainEnv = 'Development'

const { value: emailValue, errors } = useStringField({ name: 'email', rule: 'required|email' })

const handleTurnNotification = async () => {
  const config = newEvmConfig('ETHEREUM', accountAddress, tenantId, blockchainEnv)
  const client = newEvmClient(config)

  await client.logIn({
    // replace with your chain
    walletBlockchain: 'ETHEREUM',
    signMessage: async (message) => {
      // this signMessage differs by chain
      const provider = walletStrategy.getWeb3().givenProvider as any
      const { result } = await provider.send('personal_sign', ['0x' + Buffer.from(message).toString('hex'), accountAddress.toLowerCase()])
      return Buffer.from(result.replace('0x', ''), 'hex')
    }
  })

  await client.ensureTargetGroup({
    name: 'Default',
    emailAddress: emailValue.value
  })

  const alert = await client.fetchSubscriptionCard({
    id: '41b05896679f4cc1a08753f4b16a6fe9',
    type: 'SUBSCRIPTION_CARD'
  })
  await client.ensureAlert({ eventType: alert.eventTypes[0], inputs: {} })

  onClose()
  success({ title: 'Success' })
}
</script>

<template>
  <AppModal :show="isModalOpen" @modal:closed="onClose">
    <template #title>
      <h3 class="text-base">Turn on notifications</h3>
    </template>
    <div class="py-4">
      <div class="flex justify-center mb-4">
        <div class="bg-white p-4 rounded-xl">
          <img class="w-24" src="/images/notify.svg" />
        </div>
      </div>
      <p class="text-sm w-72 text-center mb-4 text-qwerty-secondary2">
        With Notifi you can enable transaction notifications, which will allow you to stay up-to-date with the status of your transactions. Never miss
        an important transaction events!
      </p>
      <AppInput v-model="emailValue" label="E-mail" />
    </div>
    <AppButton
      lg
      :disabled="emailValue == '' || errors.length > 0"
      class="w-full bg-qwerty-primary text-qwerty-background font-semibold"
      @click="handleTurnNotification"
    >
      Turn on!
    </AppButton>
  </AppModal>
</template>
