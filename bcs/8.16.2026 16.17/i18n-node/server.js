// i18n-node/server.js
const os = require('os');
const fs = require('fs');
const path = require('path'); // این ماژول برای آدرس‌دهی اضافه شد

function isMusl() {
  try {
    return fs.readFileSync('/etc/os-release', 'utf8').toLowerCase().includes('alpine') ||
           fs.readFileSync('/usr/bin/ldd', 'utf8').includes('musl');
  } catch (e) {
    return false;
  }
}

const platform = os.platform();
const arch = os.arch();
let nativeBinding = null;

if (platform === 'linux' && arch === 'x64') {
  nativeBinding = isMusl()
     ? require('./index.linux-x64-musl.node')
     : require('./index.linux-x64-gnu.node');
} else if (platform === 'win32' && arch === 'x64') {
  nativeBinding = require('./index.win32-x64-msvc.node');
} else {
  throw new Error(`Unsupported OS/Arch: ${platform}-${arch}`);
}

// --- اضافه کردن متد Async به کلاس نیتیو ---
if (nativeBinding && nativeBinding.I18nEngine) {
  nativeBinding.I18nEngine.prototype.scanMediaFolderAsync = async function(namespace, rootPath, baseUrl) {
    try {
      const locales = await fs.promises.readdir(rootPath, { withFileTypes: true });
      
      for (const dirent of locales) {
        if (dirent.isDirectory()) {
          const locale = dirent.name;
          const localePath = path.join(rootPath, locale);
          const files = await fs.promises.readdir(localePath, { withFileTypes: true });
          
          for (const file of files) {
            if (file.isFile()) {
              const fileName = file.name;
              const ext = path.extname(fileName);
              const key = path.basename(fileName, ext);
              const url = `${baseUrl}/${locale}/${fileName}`;
              
              this.addMedia(locale, namespace, key, url);
            }
          }
        }
      }
    } catch (err) {
      console.error(`[I18nEngine Error]: Failed to scan media folder asynchronously: ${err.message}`);
    }
  };
}

module.exports = nativeBinding;