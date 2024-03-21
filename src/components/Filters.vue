<script setup>
import { computed, ref, onMounted, watchEffect } from 'vue'
import { useRoute } from 'vue-router'
import { useStore } from '@src/store/store'
import { track } from '@src/utils/track'

import IconFolder from '@src/icons/icon_folder.vue'
import IconFile from '@src/icons/icon_document-text.vue'

const store = useStore()
const route = useRoute()
const filter = ref('recent')

// Computed
const filters = computed(() => store.filters)

// Watchers
watchEffect(async () => {
  if (store.filter == null) return
  filter.value = store.filter
})

// Lifecycle
onMounted(async () => await init())

// Methods
const init = async () => {
  if (store.filter == null) return
  filter.value = store.filter
}
const show = () => {
  if (route == null || !store.show.projects) return false
  return route.path !== '/settings'
}
const update = async (name) => {
  filter.value = name
  store.saveSetting({ key: 'filter', value: name })
  await track('filter_set', { name })
}
</script>

<template>
  <div v-if="show()" class="mx-auto w-[99%]">
    <ul class="grid grid-cols-3 items-center text-center">
      <li v-for="(name, index) in filters" :key="name">
        <button
          class="flex items-center justify-center gap-1.5 filter"
          :class="{
            'filter-active': filter === name,
            'rounded-l-lg': index === 0,
            'border-x-0': index > 0 && index < filters.length - 1,
            'rounded-r-lg': index === filters.length - 1,
          }"
          :aria-label="$t(`projects.filters.${name}.aria`)"
          @click.prevent="update(name)"
        >
          <IconFolder class="h-4 w-4" v-if="name != 'files'" />
          <IconFile class="h-4 w-4" v-if="name == 'files'" />
          <span>{{ $t(`projects.filters.${name}.label`) }}</span>
        </button>
      </li>
    </ul>
  </div>
</template>
