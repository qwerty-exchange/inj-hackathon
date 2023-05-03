<template>
  <div class="loading-spinner">
    <div />
    <div />
    <div />
    <AssetLogoMini class="mt-2 ml-2 h-8 w-8" />
  </div>
</template>

<style scoped>
.loading-spinner {
  display: inline-block;
  position: relative;
  width: 48px;
  height: 48px;
}
.loading-spinner div {
  box-sizing: border-box;
  display: block;
  position: absolute;
  width: 48px;
  height: 48px;
  border: 2px solid #f6be00;
  border-radius: 50%;
  animation: loading-spinner 1.2s cubic-bezier(0.5, 0.4, 0.8, 1) infinite;
  border-color: #f6be00 #f6be00 transparent transparent;
}

.loading-spinner .logo-mini {
  margin-top: 9px;
  margin-left: 8px;
}

.loading-spinner div:nth-child(1) {
  animation-delay: -0.3s;
}
.loading-spinner div:nth-child(2) {
  animation-delay: -0.55s;
}
.loading-spinner div:nth-child(3) {
  animation-delay: -0.8s;
}

@keyframes loading-spinner {
  0% {
    transform: rotate(0deg);
  }
  100% {
    transform: rotate(360deg);
  }
}
</style>
