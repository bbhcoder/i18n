const os = require('os');
const fs = require('fs');

function isMusl() {
  try {
    // روش امن و استاندارد برای تشخیص توزیع‌های لینوکسی (جلوگیری از کرش در داکر)
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

// اکسپورت صریح نام کلاس‌ها برای سازگاری کامل با سیستم ESM ناکست
module.exports = {
  I18nEngine: nativeBinding.I18nEngine,
  JsCompiledTemplate: nativeBinding.JsCompiledTemplate,
  default: nativeBinding
};