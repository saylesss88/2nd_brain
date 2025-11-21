# Floating back-to-top

Create `theme/extra.css`:

```css
/* ───── Back-to-top button ───── */
#back-to-top {
  position: fixed;
  bottom: 30px;
  right: 30px;
  z-index: 999;
  background: rgba(0, 0, 0, 0.75);
  color: white;
  width: 50px;
  height: 50px;
  line-height: 50px;
  text-align: center;
  font-size: 28px;
  border-radius: 50%;
  text-decoration: none;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.3);
  transition: all 0.3s ease;
  display: none; /* hidden until class .show is added */
}
#back-to-top.show {
  display: block;
}
#back-to-top:hover {
  background: rgba(0, 0, 0, 0.95);
  transform: scale(1.1);
}
```

Create `theme/extra.js`:

```js
// Back-to-top button
const btn = document.createElement("a");
btn.href = "#";
btn.id = "back-to-top";
btn.textContent = "Up";
btn.title = "Back to top";
document.body.appendChild(btn);

// Smooth scroll
btn.addEventListener("click", (e) => {
  e.preventDefault();
  window.scrollTo({ top: 0, behavior: "smooth" });
});

// Show/hide on scroll
window.addEventListener("scroll", () => {
  btn.classList.toggle("show", window.scrollY > 300);
});
```

Add the following to your `book.toml`:

```toml
[book]
authors = ["saylesss88"]
language = "en"
[preprocessor.toc]
renderers = ["html"]
[output.html]
additional-css = ["theme/extra.css"]
additional-js = ["theme/extra.js"]
```
