<script lang="ts" setup>
import { formatWalletAddress } from '@injectivelabs/utils'
import { ROUTES } from '@/app/utils/constants'

const route = useRoute()
const router = useRouter()
const walletStore = useWalletStore()

const { copy } = useClipboard()
const { t } = useLang()
const { success } = useNotifications()

const formattedInjectiveAddress = computed(() =>
  formatWalletAddress(walletStore.injectiveAddress)
)

function handleDisconnect() {
  walletStore.logout()

  if (ROUTES.walletConnectedRequiredRouteNames.includes(route.name as string)) {
    router.push({ name: 'index' })
  }
}

function copyAddress() {
  copy(walletStore.injectiveAddress)
  success({ title: t('connect.copiedAddress') })
}
</script>

<template>
  <div class="flex items-center">
    <BaseHoverMenu
      :skidding="-80"
      popper-class="popper bg-qwerty-shade2 rounded-lg flex flex-col flex-wrap absolute min-w-[356px] z-10 shadow-md "
    >
      <template #default="{ toggle }">
        <div
          class="font-medium text-sm cursor-pointer flex items-center justify-center lg:justify-start w-10 h-10 lg:w-auto lg:px-6 rounded-lg"
          @click="toggle"
        >
          <BaseIcon name="user" class="w-4 h-4 lg:mr-2" />
          <span class="hidden lg:block font-mono">
            {{ formattedInjectiveAddress }}
          </span>
        </div>
      </template>

      <template #content>
        <div class="rounded-t-lg p-4">
          <div class="flex items-center justify-between">
            <h3 class="text-qwerty-white text-sm font-medium">
              {{ $t('navigation.myAccount') }}
            </h3>
            <span
              class="text-qwerty-primary hover:text-opacity-80 cursor-pointer text-xs font-medium"
              @click="handleDisconnect"
            >
              {{ $t('navigation.disconnect') }}
            </span>
          </div>
          <div class="mt-2 my-2 flex items-center">
            <p class="pt-2">
              <AssetLogoMini class="w-12 h-12 mr-2" />
            </p>
            <div class="flex-1 flex-wrap">
              <div class="flex items-center justify-between w-full">
                <span
                  class="w-full block text-qwerty-white font-mono"
                  data-cy="wallet-connected-popper-inj-address-text-content"
                >
                  {{ formattedInjectiveAddress }}
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
                      @click.stop="copyAddress()"
                    />
                  </button>
                </div>
              </div>
              <LayoutWalletDetailsTierLevel />
            </div>
          </div>
          <div class="border-t border-qwerty-shade3"></div>
          <LayoutWalletDetailsConnectedWallet :wallet="walletStore.wallet" />
          <LayoutWalletDetailsSignMode />
        </div>
      </template>
    </BaseHoverMenu>
  </div>
</template>
