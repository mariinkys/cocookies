import axios, { AxiosError, type InternalAxiosRequestConfig } from 'axios'

const api = axios.create({
  baseURL: import.meta.env.VITE_API_URL ?? 'http://localhost:8080',
  withCredentials: true,
})

type RetriableConfig = InternalAxiosRequestConfig & { _retried?: boolean }

let refreshing: Promise<boolean> | null = null

/**
 * Rotates the session cookies using the refresh token. Concurrent callers share one request,
 * because the backend treats a refresh token presented twice as stolen and revokes the session.
 */
export function refreshSession(): Promise<boolean> {
  refreshing ??= api
    .post('/api/auth/refresh')
    .then(
      () => true,
      () => false,
    )
    .finally(() => {
      refreshing = null
    })
  return refreshing
}

api.interceptors.response.use(
  (response) => response,
  async (error: AxiosError) => {
    const config = error.config as RetriableConfig | undefined
    const url = config?.url ?? ''
    const isAuthEndpoint = url.includes('/api/auth/')
    const isPasswordCheck = url.includes('/password')

    if (error.response?.status !== 401 || !config || isAuthEndpoint || isPasswordCheck) {
      return Promise.reject(error)
    }

    // The short-lived access token expired: refresh once and replay the request.
    if (!config._retried && (await refreshSession())) {
      config._retried = true
      return api(config)
    }

    const isSessionCheck = url.includes('/api/users/me')
    if (!isSessionCheck) {
      const { useAuthStore } = await import('@/stores/auth')
      const auth = useAuthStore()
      auth.clearUser()
      // Guests stay where they are; the view decides what to show.
      if (!auth.isGuest && window.location.pathname !== '/login') {
        const back = window.location.pathname + window.location.search
        window.location.href = `/login?redirect=${encodeURIComponent(back)}`
      }
    }

    return Promise.reject(error)
  },
)

export default api
