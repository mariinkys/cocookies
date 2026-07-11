import { ref, onMounted } from 'vue'

const STORAGE_KEY = 'app-theme'
const isDark = ref(false)

export function useColorMode() {
  const apply = (dark: boolean) => {
    isDark.value = dark
    document.documentElement.classList.toggle('dark', dark)
    localStorage.setItem(STORAGE_KEY, dark ? 'dark' : 'light')
  }

  onMounted(() => {
    const saved = localStorage.getItem(STORAGE_KEY)
    if (saved) {
      apply(saved === 'dark')
    } else {
      apply(window.matchMedia('(prefers-color-scheme: dark)').matches)
    }
  })

  const toggle = () => apply(!isDark.value)
  const set = (dark: boolean) => apply(dark)

  return { isDark, toggle, set }
}
