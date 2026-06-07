import { definePreset } from '@primeuix/themes'
import Aura from '@primeuix/themes/aura'

export const PinkyPreset = definePreset(Aura, {
  semantic: {
    primary: {
      50: '{pink.50}',
      100: '{pink.100}',
      200: '{pink.200}',
      300: '{pink.300}',
      400: '{pink.400}',
      500: '{pink.500}',
      600: '{pink.600}',
      700: '{pink.700}',
      800: '{pink.800}',
      900: '{pink.900}',
      950: '{pink.950}',
    },

    focusRing: {
      width: '2px',
      style: 'solid',
      color: '{primary.color}',
      offset: '2px',
    },

    colorScheme: {
      light: {
        surface: {
          0: '#ffffff',
          50: '{zinc.50}',
          100: '{zinc.100}',
          200: '{zinc.200}',
          300: '{zinc.300}',
          400: '{zinc.400}',
          500: '{zinc.500}',
          600: '{zinc.600}',
          700: '{zinc.700}',
          800: '{zinc.800}',
          900: '{zinc.900}',
          950: '{zinc.950}',
        },
        primary: {
          color: '{pink.500}',
          contrastColor: '#ffffff',
          hoverColor: '{pink.600}',
          activeColor: '{pink.700}',
        },
        highlight: {
          background: '{pink.50}',
          focusBackground: '{pink.100}',
          color: '{pink.700}',
          focusColor: '{pink.800}',
        },
        formField: {
          hoverBorderColor: '{primary.color}',
          focusBorderColor: '{primary.color}',
        },
      },
      dark: {
        surface: {
          0: '#ffffff',
          50: '{slate.50}',
          100: '{slate.100}',
          200: '{slate.200}',
          300: '{slate.300}',
          400: '{slate.400}',
          500: '{slate.500}',
          600: '{slate.600}',
          700: '{slate.700}',
          800: '{slate.800}',
          900: '{slate.900}',
          950: '{slate.950}',
        },
        primary: {
          color: '{pink.400}',
          contrastColor: '{slate.950}',
          hoverColor: '{pink.300}',
          activeColor: '{pink.200}',
        },
        highlight: {
          background: 'color-mix(in srgb, {pink.400}, transparent 84%)',
          focusBackground: 'color-mix(in srgb, {pink.400}, transparent 76%)',
          color: 'rgba(255,255,255,0.87)',
          focusColor: 'rgba(255,255,255,0.97)',
        },
        formField: {
          hoverBorderColor: '{primary.color}',
          focusBorderColor: '{primary.color}',
        },
      },
    },
  },
})
