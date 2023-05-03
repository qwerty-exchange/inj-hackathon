<script lang="ts" setup>
defineProps({
  active: Boolean,
  typeIn: {
    type: String,
    required: false,
    default: ''
  }
})

const emit = defineEmits<{
  (e: 'click'): void
}>()

function handleClick() {
  emit('click')
}
</script>

<template>
  <button
    :class="{
      'justify-center uppercase flex-col mb-4': typeIn != 'block',
      'justify-between': typeIn == 'block'
    }"
    class="group relative rounded-lg py-5 px-4 items-center text-sm font-medium focus:outline-none sm:flex-1 text-qwerty-white cursor-pointer flex border border-qwerty-shade3 justify-center uppercase mb-4"
    @click.stop="handleClick"
  >
    <div class="flex flex-col gap-2 justify-center items-start">
      <slot name="label" />
      <slot name="addon" />
    </div>
    <div
      :class="{
        'border-4 border-qwerty-primary': active,
        'border border-qwerty-shade3': !active
      }"
      class="w-5 h-5 rounded-full p-[3px]"
    ></div>
  </button>
</template>
