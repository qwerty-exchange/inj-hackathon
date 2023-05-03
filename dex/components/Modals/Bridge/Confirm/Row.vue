<template>
  <div class="flex items-center justify-between">
    <span class="text-sm text-qwerty-white">
      <slot name="title" />
    </span>
    <div
      class="flex flex-col-reverse xs:flex-row items-end xs:items-center justify-between"
    >
      <span class="text-qwerty-white text-sm">
        <slot name="amountInUsd" />
      </span>
      <span class="text-qwerty-white xs:ml-2 text-sm">
        <slot name="amount" />
      </span>
    </div>
  </div>
</template>
