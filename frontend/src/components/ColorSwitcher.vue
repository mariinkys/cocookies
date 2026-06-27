<script setup lang="ts">
import { useColorMode } from '@/composables/useColorMode'
import { useThemeConfig } from '@/composables/useThemeConfig'

const { isDark, set } = useColorMode()
const { primary, neutral } = useThemeConfig()

const colors: { name: string; hex: string }[] = [
  { name: 'red', hex: '#ef4444' },
  { name: 'orange', hex: '#f97316' },
  { name: 'amber', hex: '#f59e0b' },
  { name: 'yellow', hex: '#eab308' },
  { name: 'lime', hex: '#84cc16' },
  { name: 'green', hex: '#22c55e' },
  { name: 'emerald', hex: '#10b981' },
  { name: 'teal', hex: '#14b8a6' },
  { name: 'cyan', hex: '#06b6d4' },
  { name: 'sky', hex: '#0ea5e9' },
  { name: 'blue', hex: '#3b82f6' },
  { name: 'indigo', hex: '#6366f1' },
  { name: 'violet', hex: '#8b5cf6' },
  { name: 'purple', hex: '#a855f7' },
  { name: 'fuchsia', hex: '#d946ef' },
  { name: 'pink', hex: '#ec4899' },
  { name: 'rose', hex: '#f43f5e' },
]

const neutrals: { name: string; hex: string }[] = [
  { name: 'slate', hex: '#64748b' },
  { name: 'gray', hex: '#6b7280' },
  { name: 'zinc', hex: '#71717a' },
  { name: 'neutral', hex: '#737373' },
  { name: 'stone', hex: '#78716c' },
]

// const radii = [0, 0.25, 0.5, 0.75, 1]
</script>

<template>
  <UPopover>
    <UButton
      icon="i-lucide-swatch-book"
      color="neutral"
      variant="ghost"
      size="sm"
      square
      aria-label="Customize theme"
    />

    <template #content>
      <div class="p-4 w-72 space-y-4">
        <div class="space-y-2">
          <p class="text-xs font-medium text-muted uppercase tracking-wide">Mode</p>
          <div class="flex gap-2">
            <UButton
              icon="i-lucide-sun"
              label="Light"
              size="sm"
              :color="!isDark ? 'primary' : 'neutral'"
              :variant="!isDark ? 'solid' : 'outline'"
              class="flex-1 justify-center"
              @click="set(false)"
            />
            <UButton
              icon="i-lucide-moon"
              label="Dark"
              size="sm"
              :color="isDark ? 'primary' : 'neutral'"
              :variant="isDark ? 'solid' : 'outline'"
              class="flex-1 justify-center"
              @click="set(true)"
            />
          </div>
        </div>

        <div class="space-y-2">
          <p class="text-xs font-medium text-muted uppercase tracking-wide">Primary</p>
          <div class="flex flex-wrap gap-1.5">
            <button
              v-for="color in colors"
              :key="color.name"
              :aria-label="color.name"
              :class="[
                'w-6 h-6 rounded-full transition-transform duration-100',
                primary === color.name
                  ? 'ring-2 ring-offset-2 ring-accented scale-110'
                  : 'hover:scale-110',
              ]"
              :style="{ backgroundColor: color.hex }"
              @click="primary = color.name"
            />
          </div>
        </div>

        <div class="space-y-2">
          <p class="text-xs font-medium text-muted uppercase tracking-wide">Neutral</p>
          <div class="flex flex-wrap gap-1.5">
            <button
              v-for="color in neutrals"
              :key="color.name"
              :aria-label="color.name"
              :class="[
                'w-6 h-6 rounded-full transition-transform duration-100',
                neutral === color.name
                  ? 'ring-2 ring-offset-2 ring-accented scale-110'
                  : 'hover:scale-110',
              ]"
              :style="{ backgroundColor: color.hex }"
              @click="neutral = color.name"
            />
          </div>
        </div>

        <!-- <div class="space-y-2">
          <p class="text-xs font-medium text-muted uppercase tracking-wide">Radius</p>
          <div class="flex gap-1.5">
            <button
              v-for="r in radii"
              :key="r"
              :aria-label="`Radius ${r}rem`"
              :class="[
                'flex-1 h-8 border text-xs font-medium transition-colors duration-100',
                radius === r
                  ? 'bg-primary text-(--ui-bg) border-primary'
                  : 'bg-transparent text-muted border-default hover:border-accented',
              ]"
              :style="{ borderRadius: `${r * 6}px` }"
              @click="radius = r"
            >
              {{ r }}
            </button>
          </div>
        </div> -->
      </div>
    </template>
  </UPopover>
</template>
