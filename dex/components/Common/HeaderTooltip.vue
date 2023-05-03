<script setup lang="ts">
defineProps({
  noStyling: Boolean,

  tooltip: {
    type: String,
    required: true
  }
})
</script>

<template>
  <BaseHoverMenu popper-class="tooltip">
    <template #default>
      <span
        :class="{
          'text-qwerty-secondary2 text-xs normal-case border-dashed border-b border-qwerty-shade1 cursor-pointer':
            !noStyling
        }"
      >
        <slot />
      </span>
    </template>

    <template #content>
      {{ tooltip }}
    </template>
  </BaseHoverMenu>
</template>
