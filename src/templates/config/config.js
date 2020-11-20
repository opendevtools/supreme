const config = require('@iteam/config')({
  file: `${__dirname}/../config.json`,
  defaults: {
    foo: {
      bar: 'baz',
    },
  },
})

module.exports = {
  foo: config.get('foo'),
}
