// server.js
const os = require('os');
const fs = require('fs');

// تابع تشخیص اینکه آیا لینوکس ما از نوع داکرِ آلپاین (musl) هست یا استاندارد (gnu)
function isMusl() {
  try {
    return fs.readFileSync('/usr/bin/ldd', 'utf8').includes('musl');
  } catch (e) {
    return true; // اگر دستور ldd نبود، به احتمال زیاد روی آلپاین هستیم
  }
}

const platform = os.platform();
const arch = os.arch();

let nativeBinding = null;

// تشخیص هوشمند سیستم‌عامل و معماری
if (platform === 'linux' && arch === 'x64') {
  if (isMusl()) {
    // برای داکر و Alpine Linux
    nativeBinding = require('./index.linux-x64-musl.node');
  } else {
    // برای اوبونتو، دبیان و سرورهای استاندارد
    nativeBinding = require('./index.linux-x64-gnu.node');
  }
} else if (platform === 'win32' && arch === 'x64') {
  // برای ویندوز سرور
  nativeBinding = require('./index.win32-x64-msvc.node');
} else {
  throw new Error(`معماری سیستم شما پشتیبانی نمی‌شود: ${platform} ${arch}`);
}

module.exports = nativeBinding;