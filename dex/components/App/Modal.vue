<script lang="ts" setup>
const slots = useSlots()
const { width } = useWindowSize()

const props = defineProps({
  lg: Boolean,
  md: Boolean,
  sm: Boolean,
  noPadding: Boolean,
  isAlwaysOpen: Boolean,
  hideCloseButton: Boolean
})

const emit = defineEmits<{
  (e: 'modal:closed'): void
}>()

const classes = computed(() => {
  const result = []

  if (props.sm) {
    result.push('sm:min-w-md sm:max-w-md')
  } else if (props.md) {
    result.push('md:min-w-lg md:max-w-lg', 'md:min-w-2xl lg:max-w-2xl')
  } else if (props.lg) {
    result.push('max-w-lg', 'lg:max-w-3xl')
  } else {
    result.push('max-w-lg', 'lg:max-w-4xl')
  }

  return result.join(' ')
})

function handleClose() {
  if (!props.isAlwaysOpen) {
    emit('modal:closed')
  }
}

watchDebounced(
  width,
  (newWidth, oldWidth) => {
    if (oldWidth && newWidth >= 640) {
      handleClose()
    }
  },
  { debounce: 200, immediate: true }
)
</script>

<template>
  <BaseModalWrapper
    class="relative mx-auto sm:rounded-lg bg-qwerty-shade2 max-sm:h-full max-sm:max-w-full max-sm:w-full"
    :class="classes"
    wrapper-class="backdrop-filter backdrop-blur bg-qwerty-shade1 bg-opacity-90 max-sm:z-40"
    v-bind="$attrs"
    @close="handleClose"
  >
    <template #default="{ showLoading }">
      <div
        :class="{
          'min-h-[320px] flex flex-col': showLoading
        }"
      >
        <div class="flex items-center justify-between mb-6 px-6 pt-6">
          <div
            class="text-sm uppercase text-qwerty-white font-semibold flex-grow"
          >
            <slot name="title" />
          </div>

          <div v-if="!hideCloseButton">
            <BaseIcon
              name="close"
              class="ml-auto h-5 w-5 min-w-5 text-qwerty-white hover:text-qwerty-primary"
              @click="emit('modal:closed')"
            />
          </div>
        </div>

        <div
          v-if="showLoading"
          class="grow flex items-center justify-center -mt-6"
        >
          <AppSpinner lg />
        </div>
        <div v-else>
          <div
            :class="{
              'px-6': !noPadding
            }"
          >
            <slot />
          </div>

          <div v-if="slots.footer" class="px-6">
            <slot name="footer" />
          </div>

          <div class="pb-6" />
        </div>
      </div>
    </template>
  </BaseModalWrapper>
</template>
