const Koa = require('koa');
const app = new Koa();
const chokidar = require('chokidar');
const PORT = 3000
const watch_history = ['👇record static/* files change history👇']
const ICON_EMUN = {
  'add': '📃',
  'addDir': '📂',
  'unlink': '❌',
  'change':'🌋'
}
app.use(async ctx => {
  ctx.body = `<table width=900 ><tbody>${
    watch_history.map(rec=>`<tr>${rec}</tr>`).join('')
  }</tbody></table>`
});
chokidar.watch('./static').on('all', (event, path) => {
  watch_history.push(`<td>${ICON_EMUN[event]||'🧐'}</td><td>${event}</td><td>${path}</td><td>time:${new Date().toLocaleString()}</td>`);
  console.log(event, path);
});
console.log(`server run===>  http://localhost:${PORT}`)
app.listen(PORT);