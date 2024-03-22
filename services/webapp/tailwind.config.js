module.exports = {
  content: {
      files: ["*.html", "./src/**/*.rs", "./node_modules/tw-elements/dist/js/**/*.js"],
  },
  plugins: [require("tw-elements/dist/plugin.cjs")],
  darkMode: "class"
}
