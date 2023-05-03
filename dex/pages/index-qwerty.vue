<script lang="ts" setup></script>

<template>
  <div>
    <div
      :style="{ backgroundImage: `url('/svg/bg_main.svg')` }"
      style="background-size: cover; background-repeat: no-repeat"
    >
      <PartialsHomeQwertyHero />
      <div class="h-full w-full flex flex-wrap pb-4 -mt-24">
        <div class="container">
          <div class="w-full mx-auto xl:w-4/5 py-14 bg-left-top">
            <!-- <VOverview /> -->
          </div>
        </div>
      </div>
    </div>
    <div class="h-full w-full flex flex-wrap pb-4 -mt-24">
      <div class="container">
        <div class="w-full mx-auto xl:w-4/5 py-14 bg-left-top">
          <PartialsHomeMarkets class="mt-8 sm:mt-16" />
        </div>
      </div>
    </div>
  </div>
</template>
