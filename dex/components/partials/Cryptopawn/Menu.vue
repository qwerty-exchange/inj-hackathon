<script setup lang="ts">
const FilterList = ['Market', 'Positions', 'Offers']

const props = defineProps(['modelValue'])
const emit = defineEmits(['update:modelValue'])

const value = computed({
  get() {
    return props.modelValue
  },
  set(value) {
    emit('update:modelValue', value)
  }
})
</script>

<template>
  <div class="flex items-center justify-center sm:mt-2 -mx-2 font-semibold gap-4 sm:gap-6 mb-8">
    <AppSelectButton v-for="filterType in FilterList" :key="`market-filter-type-${filterType}`" v-model="value" :value="filterType">
      <template #default="{ active }">
        <span class="text-xl hover:opacity-100" :class="[active ? 'text-qwerty-primary' : 'opacity-80']">
          <span> {{ filterType }} </span>
        </span>
      </template>
    </AppSelectButton>
  </div>
</template>
