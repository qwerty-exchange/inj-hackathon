<script lang="ts" setup>
import { PropType } from 'vue'
import { BaseDropdownOption } from '@injectivelabs/ui-shared/lib/types'

const props = defineProps({
  options: {
    type: Array as PropType<BaseDropdownOption[]>,
    required: true
  },

  modelValue: {
    type: String,
    default: ''
  },

  wrapperClass: {
    type: String,
    default: ''
  }
})

const emit = defineEmits<{
  (e: 'update:modelValue', state: string): void
}>()

const uuid = Math.random()

const selectedOption = computed(() =>
  props.options.find((option) => option.value === props.modelValue)
)

function handleSelect(option: BaseDropdownOption) {
  emit('update:modelValue', option.value)
}
</script>

<template>
  <BaseDropdown
    popper-class="selector min-w-40"
    placement="bottom-end"
    :flip="false"
  >
    <template #default="{ shown }">
      <div class="flex items-center gap-2" :class="wrapperClass">
        <slot name="prefix" />

        <slot :selected="selectedOption" />

        <slot name="icon" :shown="shown">
          <BaseIcon
            name="chevron-down"
            class="h-3 w-3 min-w-3 fill-current"
            :class="{
              'ease-in-out duration-300': shown,
              'rotate-180': shown,
              'rotate-0': !shown
            }"
          />
        </slot>
      </div>
    </template>

    <template #content="{ close }">
      <div
        class="bg-qwerty-shade2 border-qwerty-shade3 border rounded-lg p-2 flex flex-col"
      >
        <div
          v-for="(option, index) in options"
          :key="`${uuid}-selector-${index}`"
          class="flex items-center px-2 py-1 cursor-pointer rounded border-qwerty-shade3 border-b"
          :class="[
            option.value === modelValue
              ? 'text-qwerty-white hover:text-qwerty-primary hover:bg-qwerty-primary'
              : 'text-qwerty-white hover:bg-qwerty-primary hover:text-qwerty-primary'
          ]"
          @click="
            () => {
              handleSelect(option)
              close()
            }
          "
        >
          <slot name="option" :option="option" />
        </div>
      </div>
    </template>
  </BaseDropdown>
</template>
