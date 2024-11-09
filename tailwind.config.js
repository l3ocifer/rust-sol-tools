module.exports = {
  content: {
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    extend: {
      colors: {
        'primary-color': 'var(--primary-color)',
        'secondary-color': 'var(--secondary-color)',
        'background-color': 'var(--background-color)',
        'text-color': 'var(--text-color)',
        'card-background': 'var(--card-background)',
      },
      fontFamily: {
        'mono': ['var(--font-family)'],
      },
      boxShadow: {
        'glow': 'var(--glow-effect)',
      },
    },
  },
  plugins: [],
} 