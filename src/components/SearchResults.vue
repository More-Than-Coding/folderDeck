<script setup>
import { computed } from 'vue'
import { useStore } from '@src/store/store'
import List from '@src/components/List.vue'

const store = useStore()
const results = computed(() => store.searchResults)
</script>

<template>
  <List v-if="!store.show.projects" :items="results">
    <!-- No results -->
    <template v-if="store.show.noResults" #empty>
      <p class="card cursor-default select-none text-center">
        {{ $t('search.no_results') }}
      </p>
    </template>
  </List>
</template>
