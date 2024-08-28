# INFINI Pizza for WebAssembly

INFINI Pizza for WebAssembly is a fully functional search engine that can run entirely in your browser with zero dependencies. Leveraging the power and efficiency of WebAssembly, it offers optimized execution speeds far surpassing traditional JavaScript implementations.

## 🚀 Features

- **Full-Featured Query Engine**: Perform complex searches seamlessly within your browser environment.
- **Offline Indexing**: Build the search index once offline and load it directly in the browser whenever needed.
- **High Performance**: Utilize WebAssembly for optimized and efficient execution.
- **Zero Dependencies**: No external libraries or dependencies required, ensuring lightweight and fast performance.

## 🛠️ Building from Source

Follow these steps to build INFINI Pizza from source:

### 1. Prepare Dependencies

Install all necessary dependencies by running:

```sh
make init
```

### 2. Build WebAssembly Files

Compile the WebAssembly files with:

```sh
make release
make optimze
make gzip
```

After successful compilation, the `pkg` directory will contain the following files:

```sh
pkg
├── README.md
├── package.json
├── pizza-optimized.wasm
├── pizza-snipped.wasm
├── pizza_wasm.d.ts
├── pizza_wasm.js
├── pizza_wasm_bg.js
├── pizza_wasm_bg.wasm
├── pizza_wasm_bg.wasm.d.ts
└── pizza_wasm_bg.wasm.gz
```

### 3. Run the Example Web Application

Start the example web application to see INFINI Pizza in action:

```sh
make serve
```

This command will launch a local server where you can interact with the application through your browser.


## 📖 References

- **Shrinking .wasm Code Size**
   - Documentation: [Rust and WebAssembly](https://rustwasm.github.io/docs/book/reference/code-size.html)

- **Deploying WASM to Production**
   - Guide: [Rust and WebAssembly Deployment](https://rustwasm.github.io/book/reference/deploying-to-production.html)

---

Feel free to contribute to this project by submitting issues or pull requests. For any questions or support, please contact the maintainers.

**Enjoy fast and efficient search capabilities directly in your browser with INFINI Pizza!**