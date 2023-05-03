<script lang="ts" setup />

<template>
  <footer class="w-full py-16 bg-qwerty-background border-t border-t-gray-700">
    <div
      class="w-full mx-auto lg:w-4/5 grid grid-cols-1 xs:grid-cols-2 sm:grid-cols-3 3md:grid-cols-10 lg:grid-cols-12 gap-8 px-8 lg:px-0"
    >
      <div
        class="text-center sm:col-span-1 xs:col-span-2 3md:col-span-3 lg:col-span-5 lg:text-left"
      >
        <div
          class="flex items-center place-content-center cursor-pointer mb-8 lg:place-content-start"
          @click="$router.push({ name: 'index' })"
        >
          <AssetLogo class="w-36" alt="Qwerty Exchange" />
        </div>

        <span class="ml-2 text-qwerty-white text-xs">
          &copy; {{ new Date().getFullYear() }} Qwerty Exchange
        </span>
      </div>
      <div class="3md:col-span-2" />
      <div class="3md:col-span-2" />
      <div class="md:col-span-3">
        <h3 class="font-bold text-lg mb-6 text-center">
          {{ $t('footer.community') }}
        </h3>

        <div class="flex items-center place-content-center">
          <a
            class="w-6 h-6 hover:text-qwerty-primary mr-6"
            href="https://discord.gg/C9hF5axFXm"
            target="_blank"
          >
            <BaseIcon name="discord-circle" />
          </a>

          <a
            class="w-6 h-6 hover:text-qwerty-primary mr-6"
            href="https://twitter.com/QwertyExchange"
            target="_blank"
          >
            <BaseIcon name="twitter-circle" />
          </a>

          <a
            class="w-6 h-6 hover:text-qwerty-primary"
            href="https://t.me/QwertyExchange"
            target="_blank"
          >
            <BaseIcon name="telegram-circle" />
          </a>
        </div>
      </div>
    </div>
  </footer>
</template>
