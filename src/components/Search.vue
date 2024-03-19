<script setup>
import { ref, watchEffect } from 'vue'
import { watchDebounced } from '@vueuse/core'
import { useStore } from '@src/store/store'
import IconPlus from '@src/icons/icon_plus.vue'

// Setup
const store = useStore()

// Data
const search = ref('')

// Watchers
watchEffect(() => (search.value = store.search))
watchDebounced(
  search,
  async () => {
    store.search = search.value
    if (search.value.length > 0) await store.searchProjects(search.value)
    else store.searchResults = []
  },
  { debounce: 200, maxWait: 500 }
)

// Methods
const addProject = () => (store.addProject = !store.addProject)
</script>

<template>
  <section class="relative flex w-full items-center gap-2">
    <form class="flex grow items-center gap-2">
      <input
        type="search"
        spellcheck="false"
        :placeholder="$t('search.placeholder')"
        class="input w-full"
        v-model="search"
        name="q"
      />
    </form>

    <button
      class="btn-outline h-10 w-10 rounded-full p-0"
      @click.prevent="addProject()"
    >
      <IconPlus class="h-5 w-5" />
      <span class="sr-only">
        {{ $t('projects.add.button') }}
      </span>
    </button>
  </section>
</template>
