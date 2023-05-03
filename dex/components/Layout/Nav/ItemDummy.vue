<template>
  <span
    v-bind="$attrs"
    class="text-qwerty-white hover:bg-qwerty-shade3 hover:text-qwerty-primary text-sm font-semibold rounded-lg cursor-pointer mx-px h-10 flex lg:flex items-center px-6"
  >
    <span class="block">
      <slot></slot>
    </span>
  </span>
</template>
