<script lang="ts" setup>
import { PropType } from 'vue'
import { formatWalletAddress } from '@injectivelabs/utils'
import { Wallet } from '@injectivelabs/wallet-ts'

const walletStore = useWalletStore()
const { copy } = useClipboard()
const { t } = useLang()
const { success } = useNotifications()

defineProps({
  wallet: {
    required: true,
    type: String as PropType<Wallet>
  }
})

const formattedAddress = computed(() =>
  formatWalletAddress(walletStore.address)
)

function copyAddress() {
  copy(walletStore.address)
  success({ title: t('connect.copiedAddress') })
}
</script>

<template>
  <div class="my-2 flex items-center justify-between">
    <h3 class="text-qwerty-white text-sm font-medium">
      {{ $t('connect.connectWallet') }}
    </h3>
  </div>
  <div class="ml-2 my-2 flex items-center">
    <p class="pt-2">
      <BaseIcon :name="`wallet/${wallet}`" class="h-10 w-10 mr-2" />
    </p>
    <div class="flex-1 flex-wrap">
      <div class="flex items-center justify-between w-full ml-2">
        <span
          class="w-full block text-qwerty-white font-mono"
          data-cy="wallet-connected-popper-inj-address-text-content"
        >
          {{ formattedAddress }}
        </span>
        <div class="flex">
          <button
            role="button"
            type="button"
            data-cy="wallet-connected-popper-inj-address-copy-button"
          >
            <BaseIcon
              name="copy"
              class="w-5 h-5 text-qwerty-white hover:text-qwerty-primary"
              @click.stop="copyAddress"
            />
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
