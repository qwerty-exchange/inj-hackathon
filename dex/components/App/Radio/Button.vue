<script lang="ts" setup>
defineProps({
  active: Boolean
})

const emit = defineEmits<{
  (e: 'click'): void
}>()

function handleClick() {
  emit('click')
}
</script>

<template>
  <button class="flex justify-start gap-2" @click.stop="handleClick">
    <div class="w-4 h-4 border border-qwerty-shade3 rounded-full p-[3px]">
      <div v-if="active" class="w-full h-full bg-qwerty-primary rounded-full" />
    </div>
    <div class="flex flex-col gap-2 justify-center items-start">
      <slot name="label" />
      <slot name="addon" />
    </div>
  </button>
</template>
