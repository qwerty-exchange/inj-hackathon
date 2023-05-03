<script lang="ts" setup>
const props = defineProps({
  disabled: Boolean,
  modelValue: Boolean,

  tooltip: {
    type: String,
    default: ''
  },

  dataCy: {
    type: String,
    default: 'unknown-id'
  }
})

const emit = defineEmits<{
  (e: 'update:modelValue', state: boolean): void
}>()

const uid = window.crypto.getRandomValues(new Uint32Array(1))[0].toString()

const checked = computed({
  get: (): boolean => props.modelValue,
  set: (value: boolean) => {
    emit('update:modelValue', value)
  }
})
</script>

<template>
  <div class="flex items-center justify-start">
    <div class="checkbox-wrapper mr-2">
      <input
        :id="uid"
        v-model="checked"
        :disabled="disabled"
        class="checkbox"
        type="checkbox"
      />
      <label
        :for="uid"
        :data-cy="dataCy"
        class="top-0 left-0 flex items-center justify-center absolute"
        :class="{
          'cursor-pointer': !disabled
        }"
      >
        <BaseIcon
          name="check"
          class="w-3 h-3 text-qwerty-background checkmark"
        />
        <BaseIcon name="minus" class="w-3 h-3 text-qwerty-background minus" />
      </label>
    </div>
    <label
      :for="uid"
      class="select-none text-xs whitespace-nowrap"
      :class="{
        'text-qwerty-white': disabled,
        'text-qwerty-white cursor-pointer': !disabled
      }"
    >
      <slot />
    </label>
  </div>
</template>

<style scoped>
.checkbox-wrapper {
  --checkbox-width: 20px;
  --checkbox-height: 20px;
  position: relative;
  width: var(--checkbox-width);
  height: var(--checkbox-height);

  input[type='checkbox'] {
    visibility: hidden;
    width: var(--checkbox-width);
    height: var(--checkbox-height);
  }

  input[type='checkbox'] + label {
    width: var(--checkbox-width);
    height: var(--checkbox-height);
    @apply border-2 border-solid border-qwerty-shade3 rounded;
    background-color: transparent;

    .checkmark,
    .minus {
      display: none;
    }
  }

  input[type='checkbox']:checked + label {
    @apply bg-qwerty-primary;
    .checkmark {
      display: block;
    }
  }

  input[type='checkbox']:disabled + label {
    border-color: #727376;
    background-color: transparent;

    .checkmark {
      display: none;
    }

    .minus {
      display: block;
    }
  }
}
</style>
