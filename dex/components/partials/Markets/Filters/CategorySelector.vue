<script lang="ts" setup>
import { PropType } from 'vue'
import { MarketCategoryType } from '@/types'

const props = defineProps({
  active: Boolean,

  type: {
    type: String as PropType<MarketCategoryType>,
    required: true
  }
})

const emit = defineEmits<{
  (e: 'click', state: string): void
}>()

function handleTypeClick() {
  emit('click', props.type)
}
</script>

<template>
  <span
    class="text-xs cursor-pointer select-none px-3 py-2 rounded font-medium"
    :class="[
      active
        ? 'bg-qwerty-primary bg-opacity-20 text-qwerty-primary  hover:text-qwerty-primary'
        : 'text-qwerty-white  hover:text-qwerty-primary'
    ]"
    @click="handleTypeClick"
  >
    <slot />
  </span>
</template>
