/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      fontFamily: {
        sans: [
          'Inter',
          'ui-sans-serif',
          'system-ui',
          '-apple-system',
          'Segoe UI',
          'Helvetica',
          'Arial',
          'sans-serif'
        ],
        serif: ['"Instrument Serif"', 'ui-serif', 'Georgia', 'serif'],
        mono: ['ui-monospace', 'SFMono-Regular', 'Menlo', 'monospace']
      },
      colors: {
        // CMU scarlet palette (refined)
        scarlet: {
          50: '#FFF1F0',
          100: '#FFE0DC',
          200: '#FFBCB3',
          300: '#FF8E80',
          400: '#F15A49',
          500: '#C8102E',
          600: '#A50E27',
          700: '#7B0A1D'
        },
        ink: {
          50: '#F7F8FC',
          100: '#F1F3F9',
          200: '#E2E6EF',
          300: '#B7BECD',
          400: '#828CA0',
          500: '#5B6477',
          700: '#2A2F3D',
          800: '#1A1D27',
          900: '#0F1117'
        },
        emerald: {
          DEFAULT: '#10A36B'
        },
        amber: {
          DEFAULT: '#E5A100'
        }
      },
      boxShadow: {
        card: '0 1px 2px rgba(15,17,23,0.04), 0 10px 30px -20px rgba(15,17,23,0.12)',
        frame:
          '0 1px 0 rgba(15,17,23,0.04), 0 20px 50px -20px rgba(15,17,23,0.18), 0 8px 24px -12px rgba(15,17,23,0.1)',
        modal:
          '0 40px 80px -20px rgba(15,17,23,0.35), 0 10px 30px -10px rgba(15,17,23,0.2)'
      },
      keyframes: {
        pulse: {
          '0%': { 'box-shadow': '0 0 0 0 rgba(16,163,107,.5)' },
          '70%': { 'box-shadow': '0 0 0 10px rgba(16,163,107,0)' },
          '100%': { 'box-shadow': '0 0 0 0 rgba(16,163,107,0)' }
        }
      },
      animation: {
        pulse: 'pulse 1.6s ease-out infinite'
      }
    }
  },
  plugins: []
};
