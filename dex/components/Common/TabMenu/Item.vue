<script lang="ts" setup>
defineProps({
  active: Boolean
})
</script>

<template>
  <div
    class="group px-4 h-[40px] cursor-pointer relative flex items-center justify-center"
    :class="{
      'text-qwerty-white': !active,
      'text-qwerty-primary': active
    }"
  >
    <p class="text-sm capitalize select-none whitespace-nowrap">
      <slot />
    </p>
    <div
      class="group-hover:block absolute inset-0 pointer-events-none"
      :class="{
        hidden: !active
      }"
    >
      <div class="h-[2px] w-full bg-qwerty-primary absolute bottom-0" />
    </div>
  </div>
</template>
