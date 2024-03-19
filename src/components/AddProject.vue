<script setup>
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { useStore } from '@src/store/store'
import { notify } from '@src/utils/notify'

// Setup
const store = useStore()
const { t } = useI18n()

// Data
const error = ref(false)
const message = ref(null)
const project = ref(null)

const createProject = async () => {
  // Reset
  reset()

  // Create Project
  const created = await store.createProject(project.value)

  // Error - Inform User
  if (!created.success) {
    message.value = t(created.message, { project: project.value })
    error.value = true
    return
  }

  // Success - Update User
  await notify(t(created.message, { project: project.value }))
  project.value = null
}
const reset = () => {
  error.value = false
  message.value = null
}
</script>

<template>
  <Transition
    enter-active-class="duration-100 ease-out"
    enter-from-class="transform opacity-0"
    enter-to-class="opacity-100"
    leave-active-class="duration-200 ease-in"
    leave-from-class="opacity-100"
    leave-to-class="transform opacity-0"
  >
    <div
      v-if="store.addProject"
      class="scheme-main scheme-border rounded-lg border bg-opacity-30 p-4"
    >
      <h3 class="sr-only">
        {{ $t('projects.add.label') }}
      </h3>
      <!-- Form -->
      <form @submit.prevent="createProject()">
        <fieldset class="flex flex-wrap items-center gap-4">
          <label class="w-full sm:w-auto">
            <span class="sr-only">
              {{ $t('projects.add.placeholder') }}
            </span>
            <input
              type="text"
              class="input w-full"
              name="project"
              v-model="project"
              :placeholder="$t('projects.add.placeholder')"
              @input="reset()"
            />
          </label>
          <button type="submit" class="btn w-full sm:w-auto">
            {{ $t('projects.add.button') }}
          </button>
        </fieldset>
      </form>

      <!-- Warning Message -->
      <template v-if="error">
        <div class="alert alert-error mt-2 w-full">
          {{ message }}
        </div>
      </template>
    </div>
  </Transition>
</template>
