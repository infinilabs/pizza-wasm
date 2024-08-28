import { Pizza } from "pizza-wasm";
import { memory } from "pizza-wasm/pizza_wasm_bg";

// Construct the pizza engine
const pizza = Pizza.new();


//handle file upload
const fileBtn = document.getElementById('fileBtn');
fileBtn.onclick=loadFile;

//click event of load file button
async function loadFile() {
    const fileInput = document.getElementById('fileInput');
    const file = fileInput.files[0];
    if (!file) {
        alert('Please select a file!');
        return;
    }

    const reader = new FileReader();
    reader.onload = function(event) {
        const text = event.target.result;
        console.log(pizza.load(text))
    };

    reader.readAsText(file);
}



// Handle search
const searchBox = document.getElementById('queryInput');
const resultsElement = document.getElementById('results');

// Set up an event listener for input changes
searchBox.addEventListener('input', handleInputChange);

async function handleInputChange(event) {
    // Get the query string from the input box
    const query = event.target.value;

    // Call the Wasm search function
    const searchResults = pizza.search_by_query_string(query);

    const jsonString = JSON.stringify(searchResults);

    // Display the results on the page
    resultsElement.innerText = jsonString;
}



//Load index from url
async function loadFileAndIndex(url) {
    try {
        // Fetch the file from the URL
        const response = await fetch(url);

        // Check if the response is ok
        if (!response.ok) {
            throw new Error(`Failed to fetch file from ${url}: ${response.statusText}`);
        }

        // Read the file content as text
        const fileContent = await response.text();

        // Assuming `pizza` is your Wasm module object and it has a `load_text_lines` function
        const isLoaded = pizza.load_json_objects_array(fileContent);

        if (isLoaded) {
            console.log(url,' loaded and indexed successfully.');
        } else {
            console.error("Failed to load ",url);
        }
    } catch (error) {
        console.error("Error loading :",url, error);
    }
}

// Example usage:
const fileUrl = '/assets/index.json';
loadFileAndIndex(fileUrl);

