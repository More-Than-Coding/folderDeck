<script setup>
import { useI18n } from 'vue-i18n'
import { useStore } from '@src/store/store'

import IconCog from '@src/icons/icon_cog.vue'
import IconFolder from '@src/icons/icon_folder.vue'

const { t } = useI18n()
const store = useStore()
const icons = { IconCog, IconFolder }

const menu = [
  { name: t('routes.projects'), to: '/', icon: 'IconFolder' },
  { name: t('routes.settings'), to: '/settings', icon: 'IconCog' },
]
</script>

<template>
  <nav role="navigation" class="mt-4">
    <ul class="flex flex-col gap-1">
      <li v-for="(item, index) in menu" :key="index">
        <router-link
          class="flex items-center justify-center rounded-md py-2 opacity-60 hover:bg-white hover:opacity-100 dark:hover:bg-zinc-800"
          :to="item.to"
          @click="store.search = ''"
        >
          <span class="sr-only">{{ item.name }}</span>
          <component :is="icons[item.icon]" class="h-5 w-5" />
        </router-link>
      </li>
    </ul>
  </nav>
</template>
