## HTML

Create `/home/jr/projects/html/index.html`, create `~/projects/html/images/` and
place `tokyo.png` there:

```html
<!doctype html>
<html lang="en-US">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width" />
    <title>My test page</title>
  </head>
  <body>
    <img src="images/tokyo.png" alt="My test image" />
  </body>
</html>
```

Open a browser and visit: `file:///home/jr/projects/html/index.html`

## Serve over localhost (recommended)

If `/home/jr/projects/html/` contains `index.html`, run
`python -m http.server 8000` (pythons built-in server) from that directory, then
visit `http://localhost:8000` or `http://127.0.0.1:8000/` in the browser.

Bind to localhost only. If you want it accessible from your machine (not your
LAN), add a bind address:

```bash
python3 -m http.server 8000 --bind 127.0.0.1
```
