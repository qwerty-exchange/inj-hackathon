<script lang="ts" setup>
defineProps({
  alignStart: Boolean,

  title: {
    type: String,
    required: true
  }
})
</script>

<template>
  <div
    class="flex justify-between text-sm gap-4"
    :class="[alignStart ? 'items-start' : 'items-center']"
  >
    <span class="text-qwerty-white whitespace-nowrap">{{ title }}</span>
    <span class="leading-5 text-qwerty-white truncate"><slot /></span>
  </div>
</template>
