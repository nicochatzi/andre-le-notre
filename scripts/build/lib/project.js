const path = require('path');

const root = path.join(__dirname, '..', '..', '..');

module.exports = {
  settings: path.join(root, 'settings.json'),
  dirs: {
    root,
    src: path.join(root, 'src'),
    logs: path.join(root, 'logs'),
    scripts: path.join(root, 'scripts'),
  },
};
