<script setup>
const props = defineProps({
  classes: {
    type: String,
    default: 'w-6 h-6',
  },
  direction: {
    type: String,
    default: 'down',
  },
})

const orientation = () => {
  if (props.direction === 'up') return '-rotate-180'
  if (props.direction === 'right') return '-rotate-90'
  if (props.direction === 'left') return 'rotate-90'
  return ''
}
</script>
<template>
  <svg
    xmlns="http://www.w3.org/2000/svg"
    fill="none"
    viewBox="0 0 24 24"
    stroke-width="1.5"
    stroke="currentColor"
    :class="[classes, orientation()]"
    aria-hidden="true"
  >
    <path
      stroke-linecap="round"
      stroke-linejoin="round"
      d="m19.5 8.25-7.5 7.5-7.5-7.5"
    />
  </svg>
</template>
