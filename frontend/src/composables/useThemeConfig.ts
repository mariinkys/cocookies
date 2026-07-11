import { ref, watch } from 'vue'

const STORAGE_KEY = 'app-theme-config'

const primary = ref('violet')
const neutral = ref('slate')
const radius = ref(0.5)

const save = () => {
  localStorage.setItem(
    STORAGE_KEY,
    JSON.stringify({
      primary: primary.value,
      neutral: neutral.value,
      radius: radius.value,
    }),
  )
}

const applyToAppConfig = (appConfig: ReturnType<typeof useAppConfig>) => {
  appConfig.ui.colors.primary = primary.value
  appConfig.ui.colors.neutral = neutral.value
  appConfig.ui.radius = radius.value
}

export function useThemeConfig() {
  const appConfig = useAppConfig()

  const load = () => {
    const saved = localStorage.getItem(STORAGE_KEY)
    if (saved) {
      try {
        const parsed = JSON.parse(saved)
        primary.value = parsed.primary ?? 'violet'
        neutral.value = parsed.neutral ?? 'slate'
        radius.value = parsed.radius ?? 0.5
      } catch {}
    }
    applyToAppConfig(appConfig)
  }

  watch(primary, (val) => {
    appConfig.ui.colors.primary = val
    save()
  })
  watch(neutral, (val) => {
    appConfig.ui.colors.neutral = val
    save()
  })
  watch(radius, (val) => {
    appConfig.ui.radius = val
    save()
  })

  return { primary, neutral, radius, load }
}
