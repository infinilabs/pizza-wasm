# INFINI Pizza for WebAssembly

INFINI Pizza is a fully functional search engine that runs entirely in your browser with zero dependencies. Leveraging the power and efficiency of WebAssembly, it offers optimized execution speeds far surpassing traditional JavaScript implementations.

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
make build
```

After successful compilation, the `pkg` directory will contain the following files:

```sh
➜  wasm git:(main) ✗ ls -l pkg
-rw-r--r--@ 1 medcl  staff   955B Aug 23 07:37 README.md
-rw-r--r--  1 medcl  staff   424B Aug 23 13:19 package.json
-rw-r--r--  1 medcl  staff   364B Aug 23 13:19 pizza_wasm.d.ts
-rw-r--r--  1 medcl  staff   158B Aug 23 13:19 pizza_wasm.js
-rw-r--r--  1 medcl  staff    11K Aug 23 13:19 pizza_wasm_bg.js
-rw-r--r--@ 1 medcl  staff   708K Aug 23 13:19 pizza_wasm_bg.wasm
-rw-r--r--  1 medcl  staff   743B Aug 23 13:19 pizza_wasm_bg.wasm.d.ts
-rw-r--r--  1 medcl  staff   237K Aug 23 13:19 pizza_wasm_bg.wasm.gz
```

### 3. Run the Example Web Application

Start the example web application to see INFINI Pizza in action:

```sh
make serve
```

This command will launch a local server where you can interact with the application through your browser.

## 📖 Usage

To integrate and use the compiled WebAssembly module in your web projects, follow the instructions below.

### Serving Compressed WebAssembly Files

If you prefer to serve the compressed `.wasm.gz` files directly, ensure your web server is configured appropriately so that clients (e.g., web browsers) can handle them correctly.

#### **Using Nginx**

Configure Nginx to serve pre-compressed files:

```nginx
server {
    listen 80;
    server_name example.com;

    location / {
        root /path/to/your/static/files;
        gzip_static on;  # Serve .gz files directly if they exist
        types {
            application/wasm wasm;
        }
        default_type application/wasm;
        add_header Content-Encoding gzip;
    }
}
```

#### **Using Apache**

Add the following configuration to your `.htaccess` file or server configuration:

```apache
<IfModule mod_mime.c>
    AddType application/wasm .wasm
    AddEncoding gzip .gz
    AddType application/wasm .wasm.gz
</IfModule>

<IfModule mod_headers.c>
    <FilesMatch "\.wasm\.gz$">
        ForceType application/wasm
        Header set Content-Encoding gzip
    </FilesMatch>
</IfModule>
```

### Loading Compressed WebAssembly in JavaScript

To load and decompress the `.wasm.gz` file directly in the browser using JavaScript, you can utilize the [`pako`](https://github.com/nodeca/pako) library for decompression.

#### **Example Implementation**

1. **Include the `pako` library**:

   You can include `pako` via a CDN:

   ```html
   <script src="https://cdn.jsdelivr.net/npm/pako/dist/pako.min.js"></script>
   ```

2. **Load and Decompress the WebAssembly Module**:

   ```html
   <!DOCTYPE html>
   <html lang="en">
   <head>
       <meta charset="UTF-8">
       <meta name="viewport" content="width=device-width, initial-scale=1.0">
       <title>INFINI Pizza WebAssembly Demo</title>
   </head>
   <body>
       <script src="https://cdn.jsdelivr.net/npm/pako/dist/pako.min.js"></script>
       <script>
           async function loadWasm() {
               try {
                   // Fetch the compressed WASM file
                   const response = await fetch('pkg/pizza_wasm_bg.wasm.gz');
                   const compressedData = await response.arrayBuffer();

                   // Decompress the WASM file using pako
                   const decompressedData = pako.ungzip(new Uint8Array(compressedData));

                   // Instantiate the WebAssembly module
                   const wasmModule = await WebAssembly.instantiate(decompressedData.buffer);

                   console.log('WASM Module loaded successfully:', wasmModule);
                   // You can now use the exported functions from wasmModule.instance.exports
               } catch (error) {
                   console.error('Error loading WASM Module:', error);
               }
           }

           loadWasm();
       </script>
   </body>
   </html>
   ```

   **Notes**:
   - Ensure that the path to the `.wasm.gz` file is correct relative to your HTML file.
   - Handle errors appropriately to catch any issues during fetching or decompression.


## 📖 References

- **Shrinking .wasm Code Size**
   - Documentation: [Rust and WebAssembly](https://rustwasm.github.io/docs/book/reference/code-size.html)

- **Deploying WASM to Production**
   - Guide: [Rust and WebAssembly Deployment](https://rustwasm.github.io/book/reference/deploying-to-production.html)

---

Feel free to contribute to this project by submitting issues or pull requests. For any questions or support, please contact the maintainers.

**Enjoy fast and efficient search capabilities directly in your browser with INFINI Pizza!**