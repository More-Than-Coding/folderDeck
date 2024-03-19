import { createRouter, createWebHistory } from 'vue-router'

const routes = [
  {
    path: '/',
    name: 'routes.projects',
    component: () => import('../views/Projects.vue'),
  },
  {
    path: '/settings',
    name: 'routes.settings',
    component: () => import('../views/Settings.vue'),
  },
  {
    path: '/start',
    name: 'routes.start',
    component: () => import('../views/Start.vue'),
  },
]

const router = createRouter({
  history: createWebHistory(),
  routes,
})

export { routes, router }
