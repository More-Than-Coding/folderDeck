<script setup>
// Imports
import { computed, ref } from 'vue'
import { useInfiniteScroll } from '@vueuse/core'
import { UseElementVisibility } from '@vueuse/components'
import { useStore } from '@src/store/store'

// Components
import Loading from '@src/components/Loading.vue'
import Card from '@src/components/Card.vue'

const store = useStore()

// Computed
const threshold = computed(() => Math.floor(store.pagination * 0.8) - 1)

// Props
const props = defineProps({
  loading: Boolean,
  items: Array,
})

// Data
const root = ref(null)
const container = ref(null)

// Use Methods
const paginate = async () => await store.paginate()

useInfiniteScroll(container, async () => await paginate(), { distance: 10 })
</script>

<template>
  <div class="h-full p-5" ref="root">
    <!-- Loading -->
    <template v-if="loading">
      <div
        class="flex h-full cursor-default select-none flex-col items-center justify-center gap-2"
      >
        <Loading />
        <div>{{ $t('projects.setup.loading') }}</div>
      </div>
    </template>

    <!-- Items -->
    <template v-if="items && !loading">
      <div
        class="relative grid h-full snap-y snap-mandatory grid-cols-1 place-content-start gap-4 overflow-y-auto scroll-smooth pt-0.5 md:grid-cols-2 lg:grid-cols-3 2xl:grid-cols-6"
        ref="container"
      >
        <div
          class="snap-start snap-always scroll-mb-4 scroll-mt-0.5"
          v-for="(item, index) in items"
          :key="index"
        >
          <UseElementVisibility v-slot="{ isVisible }">
            <Card
              class="transition-all duration-300"
              :class="{
                'translate-y-0 scale-100': isVisible,
                'translate-y-5 scale-90': !isVisible,
              }"
              :item="item"
            />
          </UseElementVisibility>
        </div>
      </div>
    </template>

    <slot v-if="!loading" name="empty" />
  </div>
</template>
