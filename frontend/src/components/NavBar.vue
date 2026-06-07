<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRouter, useRoute } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import Button from 'primevue/button'
import VersionChip from '@/components/VersionChip.vue'
import LanguageSwitcher from '@/components/LanguageSwitcher.vue'

const { t } = useI18n({ useScope: 'global' })
const router = useRouter()
const route = useRoute()
const auth = useAuthStore()

const STORAGE_KEY = 'socios-peix-theme'
const isDark = ref(false)
const menuOpen = ref(false)

const applyTheme = (dark: boolean) => {
  isDark.value = dark
  document.documentElement.classList.toggle('my-app-dark', dark)
  localStorage.setItem(STORAGE_KEY, dark ? 'dark' : 'light')
}

onMounted(() => {
  const saved = localStorage.getItem(STORAGE_KEY)
  if (saved) {
    applyTheme(saved === 'dark')
  } else {
    applyTheme(window.matchMedia('(prefers-color-scheme: dark)').matches)
  }
})

const toggleTheme = () => applyTheme(!isDark.value)

const logout = async () => {
  await auth.logout()
  router.push('/login')
}

const isActive = (path: string) => route.path === path
</script>

<template>
  <header
    class="sticky top-0 z-50 border-b border-surface-200 dark:border-surface-700 bg-white/80 dark:bg-surface-900/80 backdrop-blur-sm transition-colors duration-200"
  >
    <div class="flex items-center justify-between px-6 h-14">
      <!-- Left Side -->
      <div class="flex items-center gap-4">
        <!-- Logo -->
        <RouterLink to="/" class="flex items-center gap-2 shrink-0 select-none">
          <img
            src="@/assets/icon.png"
            alt="Cocookies"
            class="w-7 h-7 rounded-md object-contain"
            width="28"
            height="28"
          />
          <span class="text-sm font-semibold text-surface-900 dark:text-surface-0 tracking-tight">
            Cocookies
          </span>
        </RouterLink>

        <!-- Divider, desktop only -->
        <div
          v-if="auth.isAuthenticated"
          class="hidden md:block w-px h-5 bg-surface-200 dark:bg-surface-700"
        ></div>

        <!-- Nav links, desktop only -->
        <nav v-if="auth.isAuthenticated" class="hidden md:flex items-center gap-1">
          <RouterLink
            to="/sharedrecipes"
            :class="[
              'px-3 py-1.5 rounded-md text-sm font-medium transition-colors duration-150',
              isActive('/sharedrecipes')
                ? 'bg-surface-100 dark:bg-surface-800 text-surface-900 dark:text-surface-0'
                : 'text-surface-500 dark:text-surface-400 hover:text-surface-900 dark:hover:text-surface-0 hover:bg-surface-50 dark:hover:bg-surface-800',
            ]"
          >
            {{ t('navigation.items.sharedRecipes') }}
          </RouterLink>
        </nav>

        <div v-if="!auth.isAuthenticated">
          <VersionChip />
        </div>
      </div>

      <!-- Right Side -->
      <div class="flex items-center gap-1">
        <!-- User email, desktop only -->
        <span
          v-if="auth.isAuthenticated"
          class="hidden md:inline text-xs text-surface-400 dark:text-surface-500 mr-2 max-w-[180px] truncate"
        >
          <RouterLink :to="`/users/${auth.user?.id}/edit`">
            {{ auth.user?.email }}
          </RouterLink>
        </span>

        <!-- Logout, desktop only -->
        <div class="hidden md:flex items-center" v-if="auth.isAuthenticated">
          <Button
            v-tooltip.bottom="t('common.actions.signOut')"
            icon="pi pi-sign-out"
            severity="secondary"
            text
            rounded
            size="small"
            @click="logout"
          />
        </div>

        <Button
          :icon="isDark ? 'pi pi-sun' : 'pi pi-moon'"
          :aria-label="
            isDark ? t('common.theme.switchToLightMode') : t('common.theme.switchToDarkMode')
          "
          severity="secondary"
          text
          rounded
          size="small"
          @click="toggleTheme"
        />

        <LanguageSwitcher />

        <!-- Hamburger, mobile only -->
        <div class="md:hidden" v-if="auth.isAuthenticated">
          <Button
            :icon="menuOpen ? 'pi pi-times' : 'pi pi-bars'"
            severity="secondary"
            text
            rounded
            size="small"
            :aria-label="t('common.theme.toggleMenu')"
            @click="menuOpen = !menuOpen"
          />
        </div>
      </div>
    </div>

    <!-- Mobile menu -->
    <Transition
      enter-active-class="transition-all duration-200 ease-out"
      enter-from-class="opacity-0 -translate-y-1"
      enter-to-class="opacity-100 translate-y-0"
      leave-active-class="transition-all duration-150 ease-in"
      leave-from-class="opacity-100 translate-y-0"
      leave-to-class="opacity-0 -translate-y-1"
    >
      <div
        v-if="menuOpen && auth.isAuthenticated"
        class="md:hidden border-t border-surface-200 dark:border-surface-700 bg-white dark:bg-surface-900 px-4 py-3 space-y-1"
      >
        <RouterLink
          to="/sharedrecipes"
          :class="[
            'flex items-center px-3 py-2 rounded-md text-sm font-medium transition-colors duration-150',
            isActive('/sharedrecipes')
              ? 'bg-surface-100 dark:bg-surface-800 text-surface-900 dark:text-surface-0'
              : 'text-surface-600 dark:text-surface-400 hover:bg-surface-50 dark:hover:bg-surface-800',
          ]"
          @click="menuOpen = false"
        >
          <i class="pi pi-users mr-2 text-xs"></i>
          {{ t('navigation.items.sharedRecipes') }}
        </RouterLink>

        <div
          class="pt-2 mt-2 border-t border-surface-100 dark:border-surface-800 flex items-center justify-between"
        >
          <span class="text-xs text-surface-400 dark:text-surface-500 truncate max-w-[200px]">
            {{ auth.user?.email }}
          </span>
          <Button
            :label="t('common.actions.signOut')"
            icon="pi pi-sign-out"
            severity="secondary"
            text
            size="small"
            @click="logout"
          />
        </div>
      </div>
    </Transition>
  </header>
</template>
