/** @type {import('tailwindcss').Config} */
module.exports = {
    content: {
      files: ["*.html", "./src/**/*.rs"],
    },
    theme: {
      extend: {
        transitionProperty: {
          'height': 'height',
          'width': 'width',
        },
      },
    },
    plugins: [],
  }
