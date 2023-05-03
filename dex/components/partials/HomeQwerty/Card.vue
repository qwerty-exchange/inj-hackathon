<template>
  <VCard>
    <div class="relative h-full">
      <div class="flex flex-col flex-wrap h-full">
        <div v-if="false" class="w-full">
          <p class="uppercase text-2xs text-qwerty-white">
            <slot name="category" />
          </p>
        </div>
        <div class="w-full">
          <h3 class="font-semibold text-lg text-qwerty-white">
            <slot name="title" />
          </h3>
        </div>
        <p class="text-qwerty-white tracking-tight text-2xs my-2 w-1/2">
          <slot></slot>
        </p>
        <div class="mt-auto text-xs font-semibold">
          <slot name="link" />
        </div>
      </div>
      <div class="absolute right-0 top-0">
        <slot name="illustration"></slot>
      </div>
    </div>
  </VCard>
</template>
