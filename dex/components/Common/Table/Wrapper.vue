<script lang="ts" setup>
defineProps({
  breakMd: Boolean
})

const walletStore = useWalletStore()
</script>

<template>
  <div
    class="min-h-orders flex-col h-full overflow-y-auto bg-qwerty-shade2 flex-grow"
    :class="{
      'md:min-h-0 md:flex': breakMd,
      'min-h-0 flex': !breakMd
    }"
  >
    <CommonUserNotConnectedNote
      v-if="!walletStore.isUserWalletConnected"
      cta
      class="bg-qwerty-shade2"
    />
    <slot v-else />
  </div>
</template>
