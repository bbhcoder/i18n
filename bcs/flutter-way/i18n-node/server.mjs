// i18n-node/server.mjs
import { createRequire } from 'module';
import { promises as fsPromises } from 'fs';
import path from 'path';

const require = createRequire(import.meta.url);
const nativeBinding = require('./server.js');

// استخراج کلاس‌ها از فایل نیتیو
export const I18nEngine = nativeBinding.I18nEngine;
export const JsCompiledTemplate = nativeBinding.JsCompiledTemplate;

// اضافه کردن متد غیرهمگام و Non-blocking برای اسکن مدیا
I18nEngine.prototype.scanMediaFolderAsync = async function(namespace, rootPath, baseUrl) {
  try {
    const locales = await fsPromises.readdir(rootPath, { withFileTypes: true });
    
    for (const dirent of locales) {
      if (dirent.isDirectory()) {
        const locale = dirent.name;
        const localePath = path.join(rootPath, locale);
        const files = await fsPromises.readdir(localePath, { withFileTypes: true });
        
        for (const file of files) {
          if (file.isFile()) {
            const fileName = file.name;
            const ext = path.extname(fileName);
            const key = path.basename(fileName, ext); // حذف فرمت فایل برای ساخت کلید
            const url = `${baseUrl}/${locale}/${fileName}`;
            
            // فراخوانی متد سریع و سینک Rust برای اضافه کردن به مموری
            this.addMedia(locale, namespace, key, url);
          }
        }
      }
    }
  } catch (err) {
    console.error(`[I18nEngine Error]: Failed to scan media folder asynchronously: ${err.message}`);
  }
};