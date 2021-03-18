const Koa = require('koa');
const app = new Koa();
const chokidar = require('chokidar');
const fs = require('fs');
const nativeTools = require('./lib/fw/native');
const PORT = 3001
const watch_history = [`native-->hello方法输出：${nativeTools.hello()}`,'👇record static/* files change history👇']
const ICON_EMUN = {
  'add': '📃',
  'addDir': '📂',
  'unlink': '❌',
  'change':'🌋'
}

const BASE_PATH = './static'

const filesCache = {}; // 缓存file watcher Map
app.use(async (ctx, next) => {
  const files = fs.readdirSync(BASE_PATH)
  files.map(f => {
    const _path = `${BASE_PATH}/${f}`;
    let type = false;
    let d = filesCache[_path];
    if (d) { // 已缓存，则直接拿 watcher
      if (d.version < d.watcher.version()) {
        d.version = d.watcher.version()
        type = 'change';
      }
    } else {
      const wathcer = new nativeTools.NodeFileWatcher(_path);
      d = {
        path: _path,
        watcher: wathcer,
        version: wathcer.version()
      }
      filesCache[_path] = d; // 缓存 watcher
      type = 'add';
    }
    // 有变更，则增加渲染数据===>
    type && watch_history.push(`<td>${ICON_EMUN[type]}</td><td>${type}</td><td>${_path}</td><td>${d.version}</td>`)
  })
  await next();
})


app.use(async ctx => {
  ctx.body = `<table width=900 ><tbody>${
    watch_history.map(rec=>`<tr>${rec}</tr>`).join('')
  }</tbody></table>`
});


// chokidar.watch('./static').on('all', (event, path) => {

//   const files = fs.readdirSync('./static')

//   // watcher.version()
//   // watcher.content()
//   console.log('path=====>', path)
//   const fileWatcher = new nativeTools.NodeFileWatcher(path);
//   const version = fileWatcher.version();

//   watch_history.push(`<td>${ICON_EMUN[event]||'🧐'}</td><td>${event}</td><td>${path}</td><td>${version}</td>`);
//   console.log(event, path);
// });

// console.log('nativeTools===>', nativeTools.NodeFileWatcher);
// console.log('nativeTools===>', nativeTools.getQuery);
// console.log('nativeTools===>', nativeTools.hello());


console.log(`server run===>  http://localhost:${PORT}`)
app.listen(PORT);