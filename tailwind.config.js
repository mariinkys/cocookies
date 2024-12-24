/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    relative: true,
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    extend: {},
  },
  plugins: [require("daisyui", "@tailwindcss/typography")],
  daisyui: {
    themes: [
      {
        customlight: {
          "primary": "#FF8FAB",
          "secondary": "#8FFFE3",
          "accent": "#0077B6",
          "neutral": "#118AB2",
          "base-100": "#f3f4f6",
          "info": "#118AB2",
          "success": "#06D6A0",
          "warning": "#FFD166",
          "error": "#EF476F",
        },
      },
      "dark"
    ],
    logs: false, // https://github.com/leptos-rs/cargo-leptos/issues/136
    //themes: false, // false: only light + dark | true: all themes | array: specific themes like this ["light", "dark", "cupcake"]
    darkTheme: "dark", // theme to be the default dark mode theme.
    base: true, // applies background color and foreground color for root element by default
    styled: true, // include daisyUI colors and design decisions for all components
    utils: true, // adds responsive and modifier utility classes
    prefix: "", // prefix for daisyUI classnames (components, modifiers and responsive class names. Not colors)
    logs: true, // Shows info about daisyUI version and used config in the console when building your CSS
    themeRoot: ":root", // The element that receives theme color CSS variables
  },
}