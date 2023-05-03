<template>
  <li class="block hover:bg-qwerty-shade3 cursor-pointer px-6">
    <div class="flex items-center py-4">
      <div class="min-w-0 flex-1 flex items-center">
        <div class="flex-shrink-0 mr-2 text-qwerty-white">
          <slot name="logo"></slot>
        </div>
        <div
          class="min-w-0 flex-1 px-4 md:grid md:grid-cols-1 md:gap-4 text-left"
        >
          <div>
            <p
              class="text-xl font-semibold text-qwerty-white flex items-center gap-1"
            >
              <slot name="title"></slot>
            </p>
            <div
              class="flex items-center text-base text-qwerty-white gap-2 flex-wrap"
            >
              <slot name="description"></slot>
              <slot name="addon"></slot>
            </div>
          </div>
        </div>
      </div>
      <div>
        <slot name="icon">
          <BaseIcon
            name="caret-down"
            class="transform -rotate-90 h-5 w-5 text-qwerty-white"
          />
        </slot>
      </div>
    </div>
  </li>
</template>
