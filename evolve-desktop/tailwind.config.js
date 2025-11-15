/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        primary: '#5b63ea',
        secondary: '#9b83f5',
        accent: '#37cdbe',
        neutral: '#3d4451',
        'base-100': '#ffffff',
        'base-200': '#f9fafb',
        'base-300': '#f3f4f6',
        'base-content': '#1f2937',
        info: '#3abff8',
        success: '#36d399',
        warning: '#fbbd23',
        error: '#f87272',
      },
    },
  },
  plugins: [
    require('daisyui'),
  ],
  daisyui: {
    themes: [
      {
        light: {
          "primary": "#5b63ea",
          "secondary": "#9b83f5",
          "accent": "#37cdbe",
          "neutral": "#3d4451",
          "base-100": "#ffffff",
          "base-200": "#f9fafb",
          "base-300": "#f3f4f6",
          "info": "#3abff8",
          "success": "#36d399",
          "warning": "#fbbd23",
          "error": "#f87272",
        },
        dark: {
          "primary": "#6b73f7",
          "secondary": "#ab93ff",
          "accent": "#41e5cd",
          "neutral": "#2a2e37",
          "base-100": "#1f2937",
          "base-200": "#1a1f2e",
          "base-300": "#151a27",
          "info": "#3abff8",
          "success": "#36d399",
          "warning": "#fbbd23",
          "error": "#f87272",
        },
      },
    ],
    darkTheme: "dark",
    base: true,
    styled: true,
    utils: true,
    logs: false,
  },
}
