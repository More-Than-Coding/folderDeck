<script setup>
import {
  computed,
  onBeforeMount,
  onMounted,
  onUnmounted,
  ref,
  watch,
  watchEffect,
} from 'vue'
import { useDebounceFn } from '@vueuse/core'
import { router } from '@src/plugins/router'

// Store
import { useStore } from '@src/store/store'

// Tauri
import { restoreStateCurrent, StateFlags } from 'tauri-plugin-window-state-api'
import { watchImmediate } from 'tauri-plugin-fs-watch-api'
import { getName } from '@tauri-apps/api/app'

// Utilities
import { pathFilename } from '@src/utils/paths'
import { ignoreList } from '@src/utils/entries'

// Components
import SearchResults from '@src/components/SearchResults.vue'
import Navigation from '@src/components/Navigation.vue'
import AddProject from '@src/components/AddProject.vue'
import Filters from '@src/components/Filters.vue'
import Search from '@src/components/Search.vue'
import Update from '@src/components/Update.vue'

const store = useStore()

// Data
const watcher = ref(null)
const appName = ref(null)
const showUpdate = ref(true)

// Computed
const showStart = computed(() => store.start)

// Watchers
watch(showStart, () => setRoute())
watchEffect(() => {
  if (!store.appUpdate.complete) return
  setTimeout(() => (showUpdate.value = false), 500)
})

// Lifecycle
onBeforeMount(async () => {
  restoreStateCurrent(StateFlags.SIZE)
  appName.value = await getName()
  await store.updateApp()
})
onMounted(async () => {
  await store.init()
  await store.getProjects()
  await watchUpdates()
  setRoute()
})
onUnmounted(() => {
  if (watcher.value != null) watcher.value()
})

// Methods
const setRoute = () => {
  if (showStart.value) router.push('/start')
  else router.push('/')
}
const watchUpdates = async () => {
  if (store.dir.projects == null) return

  const ignore = await ignoreList()
  const debounceUpdate = useDebounceFn(
    async () => await store.getProjects(),
    100
  )

  watcher.value = await watchImmediate(
    store.dir.projects,
    async (event) => {
      const { paths, type } = event
      const matches = paths.map((path) =>
        ignore.includes(pathFilename(path, true))
      )

      // Conditions to early exit
      if (type.access != null || matches.includes(true)) return

      // Fetch new
      await debounceUpdate()
    },
    { recursive: true }
  )
}
</script>

<template>
  <div
    class="scheme-main flex h-full w-full flex-col justify-stretch"
    :class="{ 'cursor-wait': store.loading }"
  >
    <!-- Header -->
    <header
      class="select-default flex h-8 w-full shrink-0 cursor-default select-none items-center justify-center pl-10 text-xs font-semibold uppercase tracking-wide text-neutral-500 dark:text-zinc-500"
      data-tauri-drag-region
    >
      <template v-if="$route.name && !showStart">
        {{ $t(`${$route.name}`) }}
        <template v-if="store.totalProjects > 0 && $route.path != '/settings'">
          ({{ store.totalProjects }})
        </template>
      </template>
    </header>

    <!-- Main Content -->
    <div
      class="flex h-full overflow-hidden pr-4"
      :class="{ 'pl-4': showStart }"
    >
      <!-- Navigation -->
      <aside v-if="!showStart" class="sticky top-0 w-[3.25rem] shrink-0 px-1.5">
        <Navigation />
      </aside>

      <!-- Content -->
      <main
        role="main"
        class="scheme-content relative flex-auto overflow-hidden rounded-xl"
      >
        <!-- Show updating -->
        <transition
          leave-active-class="duration-100 ease-in transform"
          leave-from-class="opacity-100"
          leave-to-class="opacity-0"
        >
          <Update v-if="showUpdate" />
        </transition>

        <!-- Content -->
        <div class="flex h-full flex-col">
          <!-- Floating Head -->
          <div
            v-if="!showStart"
            class="scheme-content sticky top-0 z-10 shrink-0 space-y-4 border-b border-neutral-100 p-5 dark:border-stone-900"
          >
            <Search />
            <AddProject />
            <Filters />
          </div>

          <!-- Router & Search Results -->
          <div class="no-scrollbar flex-grow overflow-y-auto">
            <!-- Section Content -->
            <router-view v-slot="{ Component }">
              <transition
                enter-active-class="duration-100 ease-out"
                enter-from-class="transform opacity-0"
                enter-to-class="opacity-100"
                leave-active-class="duration-200 ease-in"
                leave-from-class="opacity-100"
                leave-to-class="transform opacity-0"
              >
                <component :is="Component" />
              </transition>
            </router-view>

            <!-- Search Results -->
            <transition
              enter-active-class="duration-100 ease-out"
              enter-from-class="transform opacity-0"
              enter-to-class="opacity-100"
              leave-active-class="duration-200 ease-in"
              leave-from-class="opacity-100"
              leave-to-class="transform opacity-0"
            >
              <SearchResults v-if="!showStart" />
            </transition>
          </div>
        </div>
      </main>
    </div>

    <footer
      class="flex h-7 w-full cursor-default select-none items-center justify-center gap-2 text-[.6rem] font-semibold uppercase tracking-wide text-neutral-500 dark:text-zinc-500"
      :class="{ 'pl-10': !showStart }"
    >
      <span>{{ appName }}</span>
      <template v-if="store.lastUpdated != null">
        <span>•</span>
        <span>
          {{
            $t('general.updated', {
              time: new Date(store.lastUpdated).toLocaleTimeString(),
            })
          }}
        </span>
      </template>
    </footer>
  </div>
</template>
