import { useAuthStore } from '@/stores/auth'
import { createRouter, createWebHistory } from 'vue-router'

const routes = [
  {
    path: '/',
    name: 'Home',
    component: () => import('@/views/HomeView.vue'),
    meta: { requiresAuth: true },
  },
  // {
  //   path: '/sharedrecipes',
  //   name: 'Shared Recipes',
  //   component: () => import('@/views/recipes/SharedRecipes.vue'),
  //   meta: { requiresAuth: true },
  // },
  {
    path: '/login',
    name: 'Login',
    component: () => import('@/views/auth/LoginView.vue'),
    meta: { guestOnly: true },
  },
  {
    path: '/register',
    name: 'Register',
    component: () => import('@/views/auth/RegisterView.vue'),
    meta: { guestOnly: true },
  },
  {
    path: '/users/:id/edit',
    name: 'Update User',
    component: () => import('@/views/users/UpdateView.vue'),
    meta: { requiresAuth: true, requiresSelf: true },
  },
  {
    path: '/recipes/new',
    name: 'New Recipe',
    component: () => import('@/views/recipes/UpsertView.vue'),
    meta: { requiresAuth: true },
  },
  {
    path: '/recipes/:id/edit',
    name: 'Update Recipe',
    component: () => import('@/views/recipes/UpsertView.vue'),
    meta: { requiresAuth: true },
  },
  {
    path: '/recipes/:id/view',
    name: 'View Recipe',
    component: () => import('@/views/recipes/DetailsView.vue'),
    meta: { requiresAuth: true },
  },
]

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes,
})

router.beforeEach(async (to) => {
  const auth = useAuthStore()
  await auth.init()

  if (to.meta.requiresAuth && !auth.isAuthenticated) {
    return { name: 'Login', query: { redirect: to.fullPath } }
  }

  if (to.meta.requiresSelf) {
    const targetId = to.params.id as string
    const isSelf = auth.user?.id === targetId
    if (!isSelf) {
      return { name: 'Home' }
    }
  }

  if (to.meta.guestOnly && auth.isAuthenticated) {
    return { name: 'Home' }
  }
})

export default router
