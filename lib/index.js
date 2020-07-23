const addon = require('../native');

console.log(
    'The following functions are exported from this module:',
    addon.countWords
);

module.exports = addon;
