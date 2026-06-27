<script setup lang="ts">
import { computed, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRouter, useRoute } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import VersionChip from '@/components/VersionChip.vue'
import LanguageSwitcher from '@/components/LanguageSwitcher.vue'
import ColorSwitcher from './ColorSwitcher.vue'

const { t } = useI18n({ useScope: 'global' })
const router = useRouter()
const route = useRoute()
const auth = useAuthStore()

const mobileOpen = ref(false)

const logout = async () => {
  await auth.logout()
  router.push('/login')
}

const isActive = (path: string) => route.path === path

const navLinks = computed(() => [
  {
    label: t('navigation.items.sharedRecipes'),
    to: '/sharedrecipes',
  },
])
</script>

<template>
  <header
    class="sticky top-0 z-50 border-b border-default bg-(--ui-bg)/80 backdrop-blur-sm transition-colors duration-200"
  >
    <div class="flex items-center justify-between px-6 h-14">
      <div class="flex items-center gap-4">
        <RouterLink to="/" class="flex items-center gap-2 shrink-0 select-none">
          <img
            src="@/assets/icon.png"
            alt="Cocookies"
            class="w-7 h-7 rounded-md object-contain"
            width="28"
            height="28"
          />
          <span class="text-sm font-semibold text-highlighted tracking-tight"> Cocookies </span>
        </RouterLink>

        <template v-if="auth.isAuthenticated">
          <div class="hidden md:block w-px h-5 bg-border" />

          <nav class="hidden md:flex items-center gap-1">
            <RouterLink
              v-for="link in navLinks"
              :key="link.to"
              :to="link.to"
              :class="[
                'px-3 py-1.5 rounded-md text-sm font-medium transition-colors duration-150',
                isActive(link.to)
                  ? 'bg-elevated text-highlighted'
                  : 'text-muted hover:text-default hover:bg-elevated',
              ]"
            >
              {{ link.label }}
            </RouterLink>
          </nav>
        </template>

        <VersionChip v-if="!auth.isAuthenticated" />
      </div>

      <div class="flex items-center gap-1">
        <RouterLink
          v-if="auth.isAuthenticated"
          :to="`/users/${auth.user?.id}/edit`"
          class="hidden md:inline text-xs text-muted mr-2 max-w-45 truncate hover:text-default transition-colors"
        >
          {{ auth.user?.email }}
        </RouterLink>

        <UButton
          v-if="auth.isAuthenticated"
          class="hidden md:flex"
          :aria-label="t('common.actions.signOut')"
          icon="i-lucide-log-out"
          color="neutral"
          variant="ghost"
          size="sm"
          square
          @click="logout"
        />

        <ColorSwitcher />

        <LanguageSwitcher />

        <!-- Hamburger (mobile) -->
        <UButton
          v-if="auth.isAuthenticated"
          class="md:hidden"
          :icon="mobileOpen ? 'i-lucide-x' : 'i-lucide-menu'"
          :aria-label="t('common.theme.toggleMenu')"
          color="neutral"
          variant="ghost"
          size="sm"
          square
          @click="mobileOpen = !mobileOpen"
        />
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
        v-if="mobileOpen && auth.isAuthenticated"
        class="md:hidden border-t border-default bg-default px-4 py-3 space-y-1"
      >
        <RouterLink
          v-for="link in navLinks"
          :key="link.to"
          :to="link.to"
          :class="[
            'flex items-center gap-2 px-3 py-2 rounded-md text-sm font-medium transition-colors duration-150',
            isActive(link.to)
              ? 'bg-elevated text-highlighted'
              : 'text-muted hover:bg-elevated hover:text-default',
          ]"
          @click="mobileOpen = false"
        >
          <UIcon name="i-lucide-users" class="w-3.5 h-3.5" />
          {{ link.label }}
        </RouterLink>

        <div class="pt-2 mt-2 border-t border-muted flex items-center justify-between">
          <RouterLink
            :to="`/users/${auth.user?.id}/edit`"
            class="text-xs text-muted truncate max-w-50 hover:text-default transition-colors"
            @click="mobileOpen = false"
          >
            {{ auth.user?.email }}
          </RouterLink>

          <UButton
            :label="t('common.actions.signOut')"
            icon="i-lucide-log-out"
            color="neutral"
            variant="ghost"
            size="sm"
            @click="logout"
          />
        </div>
      </div>
    </Transition>
  </header>
</template>
