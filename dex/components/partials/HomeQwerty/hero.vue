<script lang="ts" setup>
const router = useRouter()

const walletStore = useWalletStore()
const bus = useEventBus<string>('wallet-click')

function handleGetStartedClick() {
  if (walletStore.isUserWalletConnected) {
    router.push({
      name: 'futures-futures',
      params: { futures: 'btc-usdt-perp' }
    })
  } else {
    bus.emit()
  }
}
</script>
<template>
  <div class="pt-16 pb-24 bg-cover mt-24">
    <div class="container pb-12">
      <div class="flex w-full mx-auto xl:w-4/5 justify-between">
        <div
          class="flex flex-col items-start justify-center max-w-xl lg:max-w-3xl"
        >
          <h1
            class="font-bold text-4xl leading-10 2xl:text-5xl 2xl:leading-14 tracking-wide"
          >
            {{ $t('home.title') }}
          </h1>
          <p class="text-qwerty-white mt-4 text-lg leading-6 tracking-wide">
            {{ $t('home.subtitle') }} <br />
            {{ $t('home.subsubtitle') }}
          </p>
          <AppButton
            lg
            primary
            class="min-w-4xs mt-12"
            @click="handleGetStartedClick"
          >
            {{ $t('home.getStarted') }}
          </AppButton>
        </div>
        <img
          src="/images/home_block.png"
          class="hidden w-sm h-sm md:block lg:w-xl lg:h-md"
        />
      </div>
    </div>
  </div>
</template>
