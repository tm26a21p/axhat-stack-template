/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./templates/**/*.{html,js}", "./src/**/*.{rs,html,js}"],
  theme: {
    extend: {

    },
  },
  plugins: [
    require('@tailwindcss/typography'),
  ],
  purge: {
    enabled: true,
    content: ["./templates/**/*.{html,js}", "./src/**/*.{rs,html,js}"],
  },
}
