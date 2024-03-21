<script setup>
// Vue
import { ref, reactive, watchEffect } from 'vue'
import { useStore } from '@src/store/store'

const store = useStore()

// Tauri
import { open } from '@tauri-apps/api/dialog'

// Icons
import IconDuplicate from '@src/icons/icon_document-duplicate.vue'
import IconAnalytics from '@src/icons/icon_chart-bar-square.vue'
import IconFolderOpen from '@src/icons/icon_folder-open.vue'

// Data
const analytics = ref(true)
const complete = ref(false)
const showStep = reactive({
  analytics: false,
  projects: false,
  template: false,
})

watchEffect(async () => {
  // Reset steps
  Object.keys(showStep).forEach((key) => (showStep[key] = false))

  // Show steps
  if (store.dir.projects == null) {
    showStep.projects = true
    return
  }
  if (store.dir.template == null) {
    showStep.template = true
    return
  }
  if (!showStep.analytics && !complete.value) {
    showStep.analytics = true
    return
  }

  // Setup for fresh fetch and finish start
  store.start = false
})

// Methods
const save = async ({ key, value }) => {
  if (store[key] != null) store[key] = value
  await store.saveSetting({ key, value })
}
const selectDir = async (key) => {
  const value = await open({ directory: true })
  console.log('value', value)
  // Exit if value is null
  if (value == null) return
  // Save
  store.saveSetting({ key, value, dir: true })
}
</script>
<template>
  <div class="flex h-full items-center justify-center px-20">
    <!-- Step: Projects -->
    <transition
      enter-active-class="transform-opacity duration-100 ease-out"
      enter-from-class="opacity-0 hidden"
      enter-to-class="opacity-100"
      leave-active-class="transform-opacity duration-200 ease-in"
      leave-from-class="opacity-100"
      leave-to-class="opacity-0 hidden"
    >
      <div
        v-if="showStep.projects"
        class="flex flex-col items-center justify-center"
      >
        <IconFolderOpen class="mb-2 h-[4.6rem] w-[4.6rem] opacity-60" />
        <h2 class="mb-4 text-center text-3xl font-bold">
          {{ $t('start.titles.projects') }}
        </h2>
        <p class="mb-6 text-center">{{ $t('start.info.projects') }}</p>
        <button class="btn" @click.prevent="selectDir('projects')">
          {{ $t('settings.select_projects') }}
        </button>
      </div>
    </transition>

    <!-- Step: Template -->
    <transition
      enter-active-class="transform-opacity duration-100 ease-out"
      enter-from-class="opacity-0 hidden"
      enter-to-class="opacity-100 block"
      leave-active-class="transform-opacity duration-200 ease-in"
      leave-from-class="opacity-100 block"
      leave-to-class="opacity-0 hidden"
    >
      <div
        v-if="showStep.template"
        class="flex flex-col items-center justify-center"
      >
        <IconDuplicate class="mb-2 h-[4.6rem] w-[4.6rem] opacity-60" />
        <h2 class="mb-4 text-center text-3xl font-bold">
          {{ $t('start.titles.template') }}
        </h2>
        <p class="mb-6 text-center">{{ $t('start.info.template') }}</p>
        <button class="btn" @click.prevent="selectDir('template')">
          {{ $t('settings.select_template') }}
        </button>
      </div>
    </transition>

    <!-- Step: Analytics -->
    <transition
      enter-active-class="transform-opacity duration-100 ease-out"
      enter-from-class="opacity-0 hidden"
      enter-to-class="opacity-100 block"
      leave-active-class="transform-opacity duration-200 ease-in"
      leave-from-class="opacity-100 block"
      leave-to-class="opacity-0 hidden"
    >
      <div
        v-if="showStep.analytics"
        class="flex flex-col items-center justify-center"
      >
        <IconAnalytics class="mb-2 h-[4.6rem] w-[4.6rem] opacity-60" />
        <h2 class="mb-4 text-center text-3xl font-bold">
          {{ $t('start.titles.analytics') }}
        </h2>
        <p class="mb-6 text-center">{{ $t('start.info.analytics') }}</p>
        <label class="mb-6 flex items-center gap-3">
          <input
            type="checkbox"
            class="checkbox"
            name="analytics"
            v-model="analytics"
            @change="save({ key: 'analytics', value: analytics })"
          />
          <span class="text-sm">
            {{ $t('start.labels.analytics') }}
          </span>
        </label>

        <button class="btn" @click.prevent="complete = true">
          {{ $t('start.labels.complete') }}
        </button>
      </div>
    </transition>
  </div>
</template>
