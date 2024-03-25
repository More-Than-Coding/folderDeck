<script lang="js" setup>
import { onMounted, reactive } from 'vue'
import { useStore } from '@src/store/store'
import { open } from '@tauri-apps/api/dialog'

import { defaultIgnores } from '@src/utils/entries'
import { $settings } from '@src/utils/settings'
import { openFinder } from '@src/utils/finder'

import IconFolderOpen from '@src/icons/icon_folder-open.vue'

// Constants
const store = useStore()
const ignoreId = 'ignore'

// Variables
const settings = reactive({
  analytics: true,
  ignore: null,
  pagination: 100,
})

// Lifecycle
onMounted(async () => await init())

// Methods
const init = async () => {
  const settingsArray = ['analytics', 'ignore', 'pagination']
  for (const setting of settingsArray) {
    const getSetting = await $settings.get(setting)
    if (getSetting != null) settings[setting] = getSetting.value
  }
}
const openDir = async (path) => {
  await openFinder(path)
}
const save = async ({ key, value }) => {
  if (store[key] != null) store[key] = value
  await store.saveSetting({ key, value })
}
const saveIgnores = async () => {
  // Clean the input
  settings.ignore = settings.ignore.trim()
  settings.ignore = settings.ignore.endsWith(',')
    ? settings.ignore.slice(0, -1)
    : settings.ignore

  // Save setting
  await $settings.set(ignoreId, { value: settings.settings.ignore })
  await $settings.save()

  // Reset last updated to be fetched in Projects view
  store.reset = true
}
const selectDir = async (key) => {
  const value = await open({ directory: true })

  // Exit if value is null
  if (value == null) return

  // Save and update
  await store.saveSetting({ dir: true, key, value })
}
</script>

<template>
  <div class="p-5">
    <div class="max-w-[35rem] space-y-6">
      <!-- Project Folder -->
      <div class="space-y-3">
        <div class="flex items-center gap-2">
          <button
            class="btn btn-sm px-6"
            @click.prevent="selectDir('projects')"
          >
            {{ $t('settings.select_projects') }}
          </button>
          <button
            v-if="store.dir.projects != null"
            @click.prevent="openDir(store.dir.projects)"
            class="btn-outline-solid btn-sm flex items-center gap-1.5 px-6"
            :title="$t('settings.open_dir_aria')"
          >
            <IconFolderOpen class="h-4 w-4" />
            <span>{{ $t('settings.open_dir') }}</span>
          </button>
        </div>

        <template v-if="store.dir.projects">
          <div
            class="scheme-main select-all rounded-md px-3 py-1.5 text-xs font-light uppercase tracking-[.06rem] dark:text-opacity-60"
          >
            {{ store.dir.projects }}
          </div>
        </template>
      </div>

      <!-- Template folder -->
      <div class="space-y-3">
        <div class="flex items-center gap-2">
          <button
            class="btn btn-sm px-6"
            @click.prevent="selectDir('template')"
          >
            {{ $t('settings.select_template') }}
          </button>
          <button
            v
            v-if="store.dir.template != null"
            @click.prevent="openDir(store.dir.template)"
            class="btn-outline-solid btn-sm flex items-center gap-1.5 px-6"
          >
            <IconFolderOpen class="h-4 w-4" />
            <span>{{ $t('settings.open_dir') }}</span>
          </button>
        </div>

        <template v-if="store.dir.template">
          <div
            class="scheme-main select-all rounded-md px-3 py-1.5 text-xs font-light uppercase tracking-[.06rem] dark:text-opacity-60"
          >
            {{ store.dir.template }}
          </div>
        </template>
      </div>

      <!-- Pagination -->
      <div class="space-y-3">
        <label>
          <div class="mb-2 text-sm font-medium">
            {{ $t('settings.pagination.label') }}
          </div>
          <input
            type="number"
            name="pagination"
            placeholder="100"
            @change="save({ key: 'pagination', value: settings.pagination })"
            v-model="settings.pagination"
            class="input max-w-40 text-center"
            step="10"
            max="1000"
            min="10"
          />
          <div class="pt-2 text-xs opacity-60">
            {{ $t('settings.pagination.info') }}
          </div>
        </label>
      </div>

      <!-- Ignore list -->
      <div class="space-y-3">
        <label>
          <div class="mb-2 text-sm font-medium">
            {{ $t('settings.ignore.label') }}
          </div>
          <textarea
            rows="2"
            name="ignore"
            autocomplete="off"
            spellcheck="false"
            class="input w-full"
            :placeholder="$t('settings.ignore.placeholder')"
            @change="saveIgnores()"
            v-model="settings.ignore"
          ></textarea>
          <div class="pt-2 text-xs opacity-60">
            {{
              $t('settings.ignore.info', {
                files: defaultIgnores.toString().replaceAll(',', ', '),
              })
            }}
          </div>
        </label>
      </div>

      <!-- Analytics Tracking  -->
      <div class="space-y-3">
        <label>
          <div class="flex items-center gap-3">
            <input
              type="checkbox"
              name="analytics"
              v-model="settings.analytics"
              @change="save({ key: 'analytics', value: settings.analytics })"
              class="checkbox"
            />
            <div class="font-medium">
              {{ $t('settings.analytics.label') }}
            </div>
          </div>
          <div class="pt-2 text-xs opacity-60">
            {{ $t('settings.analytics.info') }}
          </div>
        </label>
      </div>
    </div>
  </div>
</template>
