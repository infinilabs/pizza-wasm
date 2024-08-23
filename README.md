# INFINI Pizza for WebAssembly


## Why?

Run a full function search engine in your browser with zero dependencies.

WebAssembly is also far more optimized than JavaScript for execution speed.

Few things we are different
- Full functional query engine
- Build index offline once, can be loaded directly in your browser


### 🛠️ How to Build

Build wasm files:

```
make build
make zip
```

Checkout the `pkg` dir, you will see compiled wasm files:
```
➜  wasm git:(main) ✗ ll pkg 
-rw-r--r--@ 1 medcl  staff   955B Aug 23 07:37 README.md
-rw-r--r--  1 medcl  staff   424B Aug 23 13:19 package.json
-rw-r--r--  1 medcl  staff   364B Aug 23 13:19 pizza_wasm.d.ts
-rw-r--r--  1 medcl  staff   158B Aug 23 13:19 pizza_wasm.js
-rw-r--r--  1 medcl  staff    11K Aug 23 13:19 pizza_wasm_bg.js
-rw-r--r--@ 1 medcl  staff   708K Aug 23 13:19 pizza_wasm_bg.wasm
-rw-r--r--  1 medcl  staff   743B Aug 23 13:19 pizza_wasm_bg.wasm.d.ts
-rw-r--r--  1 medcl  staff   237K Aug 23 13:19 pizza_wasm_bg.wasm.gz
```

Now, start the example web application:

```
make serve
```

### How to use

If you want to serve the compressed WebAssembly binary directly in a web environment (e.g., a web server), you need to ensure that your web server is configured to serve the `.gz` file correctly and that the client (e.g., a web browser) knows how to handle it.

#### Configure Your Web Server

If you’re using a web server like Nginx or Apache, you can configure it to serve gzip-compressed WebAssembly files.

**For Nginx:**

You can use the `gzip_static` module to serve pre-compressed files:

```nginx
server {
    listen 80;
    server_name example.com;

    location / {
        root /path/to/your/static/files;
        gzip_static on;  # Serve .gz files directly if they exist
        add_header Content-Encoding gzip;
    }
}
```

**For Apache:**

Add the following configuration to your `.htaccess` file or server configuration:

```apache
<IfModule mod_mime.c>
    AddType application/wasm .wasm
    AddEncoding gzip .gz
</IfModule>
```

#### Manually **Loading `wasm.gz` in JavaScript**

To load and use a compressed WebAssembly binary in JavaScript, you need to handle the decompression process manually. This is typically done on the server-side, but you can also implement it in JavaScript using the `pako` library to decompress the WebAssembly binary.

##### Example JavaScript Code

1. **Install the `pako` library**:

   If you use a package manager like npm or yarn, you can install `pako`:

   ```sh
   npm install pako
   ```

2. **Decompress and load the WebAssembly binary**:

   ```html
   <!DOCTYPE html>
   <html lang="en">
   <head>
       <meta charset="UTF-8">
       <meta name="viewport" content="width=device-width, initial-scale=1.0">
       <title>Load WASM</title>
   </head>
   <body>
       <script src="https://cdn.jsdelivr.net/npm/pako/dist/pako.min.js"></script>
       <script>
           async function loadWasm() {
               // Fetch the gzipped WASM file
               const response = await fetch('pkg/pizza_wasm_bg.wasm.gz');
               const compressedData = await response.arrayBuffer();

               // Decompress the data using pako
               const decompressedData = pako.ungzip(new Uint8Array(compressedData));

               // Instantiate the WebAssembly module
               const wasmModule = await WebAssembly.instantiate(new Uint8Array(decompressedData));
               console.log('WASM Module:', wasmModule);
           }

           loadWasm();
       </script>
   </body>
   </html>
   ```



### Dataset

Free, simple, open recipe dataset
- https://github.com/josephrmartinez/recipe-dataset

RecipeNLG
- https://recipenlg.cs.put.poznan.pl/dataset

TMDB Movies
- https://www.kaggle.com/datasets/tmdb/tmdb-movie-metadata/data?select=tmdb_5000_movies.csv



### Work with 
-  https://github.com/xenova/transformers.js


### References

- Shrinking .wasm Code Size: https://rustwasm.github.io/docs/book/reference/code-size.html
- Deploy WASM to production: https://rustwasm.github.io/book/reference/deploying-to-production.html