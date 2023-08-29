/** @type {import('tailwindcss').Config} */
const colors = require("tailwindcss/colors");

module.exports = {
  content: ["./templates/**/*.{html,js}"],
  theme: {
    extend: {
      colors: {
        primary: colors.pink,
      },
      // fontFamily: {
      //   puddle: ["'Rubik Puddles'", "monospace"],
      // },
    },
  },
  plugins: [require("@tailwindcss/typography")],
};
