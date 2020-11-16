module.exports = {
  purge: {
    content: ['./src/**/*.bs.js'],
  },
  theme: {},
  variants: {},
  plugins: [],
  future: {
    purgeLayersByDefault: true,
    removeDeprecatedGapUtilities: true,
  },
}
