const logger = require('node-color-log');

const log = (category, str, color) =>
  logger
    .log(`${new Date().toISOString()} : `)
    .joint()
    .color(color)
    .bold()
    .log(`[ ${category} ] : `)
    .joint()
    .color(color)
    .log(str);

module.exports = {
  run: (str) => log('RUN', str, 'green'),
  error: (str) => log('ERR', str, 'red'),
  warn: (str) => log('WARN', str, 'red'),
  info: (str) => log('INFO', str, 'blue'),
};
