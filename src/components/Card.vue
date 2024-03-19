<script setup>
import { useTimeAgo } from '@vueuse/core'
import { useStore } from '@src/store/store'

// Utilities
import { openFinder } from '@src/utils/finder'
import { fileType, fileSize } from '@src/utils/files'

// Icons
import IconClock from '@src/icons/icon_clock.vue'
import IconFolder from '@src/icons/icon_folder.vue'
import IconFile from '@src/icons/icon_document-text.vue'
import IconAudio from '@src/icons/icon_speaker-wave.vue'
import IconPhoto from '@src/icons/icon_photo.vue'
import IconVideo from '@src/icons/icon_camera.vue'

// Setup
const store = useStore()

// Props
const props = defineProps({
  item: Object,
})

// Methods
const projectName = (item) => {
  if (item.metadata.is_dir) return
  return item.path.replace(store.dir.projects, '').split('/')[1]
}
</script>

<template>
  <div class="card group relative w-full space-y-5 text-left">
    <!-- Columns -->
    <div class="grid grid-cols-12 gap-5">
      <!-- Column 1 -->
      <div class="col-span-3">
        <!-- Icon -->
        <div
          class="pill w-full py-2 opacity-75 group-hover:opacity-100"
          :class="{
            'pill-dir': item.metadata.is_dir,
            'pill-file': fileType(item)?.other,
            'pill-audio': fileType(item)?.audio,
            'pill-image': fileType(item)?.image,
            'pill-video': fileType(item)?.video,
          }"
        >
          <IconFolder class="h-5 w-5" v-if="item.metadata.is_dir" />
          <IconAudio class="h-5 w-5" v-if="fileType(item)?.audio" />
          <IconPhoto class="h-5 w-5" v-if="fileType(item)?.image" />
          <IconFile class="h-5 w-5" v-if="fileType(item)?.other" />
          <IconVideo class="h-5 w-5" v-if="fileType(item)?.video" />
        </div>
      </div>

      <!-- Column 2 -->
      <div class="col-span-9 space-y-1.5">
        <!-- Title -->
        <div class="flex items-center gap-3">
          <h3 class="w-full truncate font-bold text-black dark:text-white">
            {{ item.name }}
          </h3>
        </div>

        <!-- Metadata -->
        <div class="grid grid-cols-5 items-center gap-1 text-sm opacity-80">
          <div
            v-if="item.metadata.is_file"
            class="col-span-3 flex items-center gap-1.5 text-sm"
          >
            <IconFolder class="h-[1.1rem] w-[1.1rem]" />
            <span class="truncate">{{ projectName(item) }}</span>
          </div>

          <div
            v-if="item.metadata.file_size != null"
            class="col-span-2 text-right"
          >
            {{ fileSize(item.metadata.file_size, 1) }}
          </div>

          <div class="col-span-full flex items-center gap-1.5 italic">
            <IconClock class="h-[1.1rem] w-[1.1rem]" />
            <span>
              {{ useTimeAgo(new Date(item.metadata.modified)).value }}
            </span>
          </div>
        </div>
      </div>
    </div>

    <!-- Action Buttons -->
    <div class="flex items-center gap-2">
      <button
        class="btn-outline-solid btn-sm w-full"
        @click.prevent="openFinder(item.path)"
      >
        <template v-if="item.metadata.is_dir">
          {{ $t('projects.action.open_dir') }}
        </template>
        <template v-else>
          {{ $t('projects.action.open_file') }}
        </template>
      </button>
    </div>
  </div>
</template>
