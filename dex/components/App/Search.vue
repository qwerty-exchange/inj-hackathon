<script lang="ts" setup>
const props = defineProps({
  sm: Boolean,
  showPrefix: Boolean,

  inputClasses: {
    type: String,
    default: ''
  }
})

const inputClass = computed(() => {
  const result = []

  if (props.showPrefix) {
    result.push('px-2')
  }

  result.push(props.inputClasses)

  return result.join(' ')
})
</script>

<template>
  <AppInput
    v-bind="$attrs"
    ref="search-input"
    :sm="sm"
    :input-classes="inputClass"
  >
  </AppInput>
</template>
