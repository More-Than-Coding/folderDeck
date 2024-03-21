<script setup>
import { computed, nextTick, onMounted } from 'vue'
import { useStore } from '@src/store/store'
import List from '@src/components/List.vue'

// Setup
const store = useStore()

// Lifecycle
onMounted(async () => await store.resetProjects())

// Computed
const projects = computed(() => {
  if (store.filter == null) return
  const projectsBy = {
    name: store.sortProjectsName,
    recent: store.sortProjectsRecent,
    files: store.sortFilesRecent,
  }
  return projectsBy[store.filter]
})
</script>

<template>
  <List v-if="store.show.projects" :items="projects" :loading="!store.ready">
    <!-- No projects -->
    <template v-if="!store.show.projects && store.search.length === 0" #empty>
      <p class="card cursor-default select-none text-center">
        {{ $t('projects.empty') }}
      </p>
    </template>
  </List>
</template>
