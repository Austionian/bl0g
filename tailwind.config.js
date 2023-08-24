/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./templates/**/*.{html,js}"],
  theme: {
    extend: {
      fontFamily: {
        puddle: ["'Rubik Puddles'", "monospace"],
      },
    },
  },
  plugins: [],
};
