import { defineStore } from 'pinia'

// Tauri
import { invoke } from '@tauri-apps/api'

// Utils
import { timeConvertMs } from '@src/utils/performance'
import { ignoreList } from '@src/utils/entries'
import { $settings } from '@src/utils/settings'
import { track } from '@src/utils/track'
import { devLog } from '@src/utils/log'

// Variables
export const useStore = defineStore('main', {
  state: () => ({
    addProject: false,
    dir: {
      projects: null,
      template: null,
    },
    filter: 'recent',
    filters: ['recent', 'name', 'files'],
    lastUpdated: null,
    loading: false,
    pagination: 100,
    ready: false,
    start: false,
    search: '',
    searchResults: [],
    sortProjectsName: [],
    sortProjectsRecent: [],
    sortFilesRecent: [],
    totalProjects: 0,
  }),
  getters: {
    show(state) {
      return {
        projects: state.searchResults.length === 0 && state.search.length === 0,
        noResults: state.searchResults.length === 0 && state.search.length > 0,
        searchResults:
          state.searchResults.length > 0 && state.search.length > 0,
      }
    },
  },
  actions: {
    async init() {
      // Get data values
      const pagination = await $settings.get('pagination')
      const projects = await $settings.get('projects')
      const template = await $settings.get('template')
      const filter = await $settings.get('filter')

      // Exit if blank
      if (projects == null || template == null) {
        this.start = true
        return
      }

      // Setup Directories
      this.dir.projects = projects.value
      this.dir.template = template.value

      // Update pagination
      if (pagination != null) this.pagination = pagination.value
      if (filter != null) this.filter = filter.value
    },
    async createProject(projectName) {
      // Early exit if project is blank
      if (projectName == null || projectName.length < 1) return result

      const result = {
        success: false,
        message: null,
      }

      // Copy directory
      const res = await invoke('copy_directory', {
        source: this.dir.template,
        destination: `${this.dir.projects}/${projectName}`,
      })

      // Failed as directory already exists
      if (!res.success && res.exists) {
        result.message = 'projects.alerts.exists'
        return result
      }

      // Failed to unknown error
      if (!res.success && !res.exists) {
        result.message = 'projects.alerts.error'
        return result
      }

      // Update projects
      await this.getProjects()

      // Return Success
      result.success = true
      result.message = 'projects.alerts.success'
      return result
    },
    async getProjects() {
      // Determine if should block fetching
      if (this.dir.projects == null || this.dir.template == null) return

      // Start
      this.loading = true

      // Performance tracking start
      const timeStart = performance.now()

      // // RUST get files and metadata
      const ignore = await ignoreList()
      const entries = await invoke('read_directory', {
        path: this.dir.projects,
        ignore,
      })
      this.totalProjects = entries.projects

      // Testing fetch cache
      const projectsName = await invoke('projects_name', {
        page: 0,
        pageSize: this.pagination,
      })
      const projectsRecent = await invoke('projects_recent', {
        page: 0,
        pageSize: this.pagination,
      })
      const filesRecent = await invoke('files_recent', {
        page: 0,
        pageSize: this.pagination,
      })

      this.sortProjectsName = projectsName.items
      this.sortProjectsRecent = projectsRecent.items
      this.sortFilesRecent = filesRecent.items

      // Performance tracking start
      const timeEnd = performance.now()
      const timeComplete = timeEnd - timeStart

      // Finish
      this.lastUpdated = new Date()
      this.loading = false
      if (!this.ready) this.ready = true

      // Analytics tracking of project load times
      await track('load_projects', { time_ms: timeComplete })

      // Log if in dev
      devLog({ title: '📂 Projects', message: `${entries.projects} total` })
      devLog({ title: '🕛 Time Elapsed', message: timeConvertMs(timeComplete) })
    },
    async saveSetting({ dir = false, key, value }) {
      // Set loading
      this.loading = true

      // Update state
      if (dir) this.dir[key] = value
      else this[key] = value

      // Update store
      await $settings.set(key, { value })
      await $settings.save()

      if (key === 'projects' || key === 'pagination') {
        await invoke('reset_caches')
        this.getProjects()
      }

      // Reset loading
      this.loading = false
    },
    async searchProjects(query) {
      const timeStart = performance.now()

      this.searchResults = await invoke('search', { query, limit: 5 })

      const timeEnd = performance.now()
      const timeComplete = timeEnd - timeStart

      // Analytics tracking of project load times
      await track('search_projects', { time_ms: timeComplete })

      // Log if in dev
      devLog({
        title: '🕛 Search Response',
        message: timeConvertMs(timeComplete),
      })
    },
  },
})
