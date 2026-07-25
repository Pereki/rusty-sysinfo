/**
 * plugins/vuetify.ts
 *
 * Framework documentation: https://vuetifyjs.com
 */

import { createVuetify } from 'vuetify'
import '@mdi/font/css/materialdesignicons.css'
import 'vuetify/styles'

const lightTheme = {
    dark: false,
  colors: {
        primary: '#1e40af',
        background: '#f8fafc',
        surface: '#ffffff',
        'surface-variant': '#f1f5f9',
        'on-surface': '#0f172a',
        'primary-darken-1': '#1e293b',
        secondary: '#64748b',
        'secondary-darken-1': '#475569',
        error: '#ef4444',
        info: '#3b82f6',
        success: '#22c55e',
        warning: '#f59e0b',
    },
}

export default createVuetify({
    theme: {
        defaultTheme: 'light',
        themes: { light: lightTheme },
        utilities: false,
    },
    display: {
        mobileBreakpoint: 'md',
        thresholds: {
            xs: 0,
            sm: 600,
            md: 840,
            lg: 1145,
            xl: 1545,
            xxl: 2138,
        },
    },
})
