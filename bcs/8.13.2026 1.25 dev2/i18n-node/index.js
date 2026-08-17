// i18n-node/index.js
const { platform, arch } = process;

let nativeBinding = null;

try {
  if (platform === 'win32' && arch === 'x64') {
    nativeBinding = require('./index.win32-x64-msvc.node');
  } else if (platform === 'linux' && arch === 'x64') {
    nativeBinding = require('./index.linux-x64-gnu.node');
  } else {
    throw new Error(`Unsupported OS/Arch: ${platform}-${arch}`);
  }
} catch (err) {
  console.error("Failed to load native I18N binding:", err.message);
  throw err;
}

module.exports = nativeBinding;