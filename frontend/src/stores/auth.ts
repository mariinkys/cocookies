import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import api from '@/api/axios'
import { systemService } from '@/services/system.service'
import type { AuthResponse, LoginPayload } from '@/types/auth.types'
import type { UserResponse } from '@/types/user.types'

const GUEST_KEY = 'cocookies.guest'

function readGuestFlag(): boolean {
  try {
    return localStorage.getItem(GUEST_KEY) === '1'
  } catch {
    return false
  }
}

function writeGuestFlag(value: boolean): void {
  try {
    if (value) localStorage.setItem(GUEST_KEY, '1')
    else localStorage.removeItem(GUEST_KEY)
  } catch {
    // Storage may be unavailable (private mode); guest mode then lasts for this page only.
  }
}

export const useAuthStore = defineStore('auth', () => {
  const user = ref<AuthResponse | null>(null)
  const loading = ref(false)
  /** The visitor chose to browse without an account. */
  const guestChosen = ref(readGuestFlag())
  /** Whether the server allows read-only guest access (`GUEST_ACCESS`). */
  const guestAccessEnabled = ref(false)
  let initPromise: Promise<void> | null = null

  const isAuthenticated = computed(() => user.value !== null)
  /** A visitor browsing shared recipes read-only, without an account. */
  const isGuest = computed(
    () => !isAuthenticated.value && guestChosen.value && guestAccessEnabled.value,
  )

  async function init(): Promise<void> {
    initPromise ??= Promise.all([fetchCurrentUser(), fetchServerConfig()]).then(() => undefined)
    return initPromise
  }

  async function login(payload: LoginPayload): Promise<void> {
    const { data } = await api.post<AuthResponse>('/api/auth/login', payload)
    user.value = data
    leaveGuest()
    initPromise = Promise.resolve()
  }

  async function logout(): Promise<void> {
    try {
      await api.post<void>('/api/auth/logout')
    } finally {
      user.value = null
      leaveGuest()
      initPromise = null
    }
  }

  async function fetchCurrentUser(): Promise<void> {
    loading.value = true
    try {
      const { data } = await api.get<UserResponse>('/api/users/me')
      user.value = data
    } catch {
      user.value = null
    } finally {
      loading.value = false
    }
  }

  async function fetchServerConfig(): Promise<void> {
    try {
      guestAccessEnabled.value = (await systemService.getConfig()).guestAccess
    } catch {
      guestAccessEnabled.value = false
    }
  }

  function enterAsGuest(): void {
    guestChosen.value = true
    writeGuestFlag(true)
  }

  function leaveGuest(): void {
    guestChosen.value = false
    writeGuestFlag(false)
  }

  function clearUser(): void {
    user.value = null
    initPromise = null
  }

  return {
    user,
    loading,
    isAuthenticated,
    isGuest,
    guestAccessEnabled,
    init,
    login,
    logout,
    fetchCurrentUser,
    enterAsGuest,
    leaveGuest,
    clearUser,
  }
})
