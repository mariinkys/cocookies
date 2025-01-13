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
          "neutral": "#CFCFCF",
          "base-100": "#F3F4F6",
          "info": "#118AB2",
          "success": "#06D6A0",
          "warning": "#FFD166",
          "error": "#EF476F",
        },
        customdark: {
          "primary": "#FF8FAB",
          "secondary": "#8FFFE3",
          "accent": "#0077B6",
          "neutral": "#CFCFCF",
          "base-100": "#1F2937",
          "info": "#118AB2",
          "success": "#06D6A0",
          "warning": "#FFD166",
          "error": "#EF476F",
        },
      },
    ],
    logs: false, // https://github.com/leptos-rs/cargo-leptos/issues/136
    //themes: false, // false: only light + dark | true: all themes | array: specific themes like this ["light", "dark", "cupcake"]
    darkTheme: "customlight", // theme to be the default dark mode theme. (always default to light, if not change theme button does not work when default is dark at least on mobile)
    base: true, // applies background color and foreground color for root element by default
    styled: true, // include daisyUI colors and design decisions for all components
    utils: true, // adds responsive and modifier utility classes
    prefix: "", // prefix for daisyUI classnames (components, modifiers and responsive class names. Not colors)
    logs: true, // Shows info about daisyUI version and used config in the console when building your CSS
    themeRoot: ":root", // The element that receives theme color CSS variables
  },
}