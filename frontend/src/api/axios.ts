import axios, { AxiosError } from 'axios'

const api = axios.create({
  baseURL: import.meta.env.VITE_API_URL ?? 'http://localhost:8080',
  withCredentials: true,
})

api.interceptors.response.use(
  (response) => response,
  (error: AxiosError) => {
    const url = error.config?.url ?? ''
    const isAuthEndpoint = url.includes('/api/auth/')
    const isSessionCheck = url.includes('/api/users/me')
    const isPasswordCheck = url.includes('/password')

    if (error.response?.status === 401 && !isAuthEndpoint && !isSessionCheck && !isPasswordCheck) {
      import('@/stores/auth').then(({ useAuthStore }) => useAuthStore().clearUser())
      window.location.href = '/login'
    }

    return Promise.reject(error)
  },
)

export default api
