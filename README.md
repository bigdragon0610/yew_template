# Versions
```
rustc: 1.66.0
Yew: 0.20
trunk: 0.16.0
tailwindcss: ^3.2.4
```

# Setup
1. 事前準備
```sh
rustup target add wasm32-unknown-unknown # wasm
cargo install --locked trunk # trunk
cargo new yew-app # プロジェクト名をつける
```

2. `Cargo.toml` の dependencies に以下を追加
```toml
yew = { version = "0.20", features = ["csr"] }
```

3. tailwindcssの導入
```sh
npm install -D tailwindcss
npx tailwindcss init
```

4. `tailwind.config.js` の module.exports に以下を追加
```js
content: ["./src/**/*.rs"],
```

5. ./src/css に input.css と output.css を追加、input.css に以下を追加
```css
@tailwind base;
@tailwind components;
@tailwind utilities;
```

6. vscode の設定をする
```sh
# Emmet: Include Languages
"rust": "html"

# 先に Tailwind Css Intellisense をインストールしておく
# Tailwind CSS: Include Languages
"rust": "html"
```

7. プロジェクトルートに index.html を追加し、以下のように設定
``` html
<!DOCTYPE html>
<html lang="ja">
  <head>
    <!--　link タグに data-trunk と rel="css" をつける -->
    <link data-trunk rel="css" href="/src/css/output.css" />
    <title>Yew App</title>
  </head>
  <body></body>
</html>

```

8. 以下のコマンドを実行して開発を開始
```sh
trunk serve
npx tailwindcss -i ./src/css/input.css -o ./src/css/output.css --watch
```

