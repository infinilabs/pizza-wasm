import { Pizza } from "pizza-wasm";
import { memory } from "pizza-wasm/pizza_wasm_bg";

/**
 * Initialize the Pizza search engine
 * This is a WASM-based search engine that provides fast text search capabilities
 */
const pizza = Pizza.new();

// Get DOM elements
const searchBox = document.getElementById('queryInput');
const resultsElement = document.getElementById('results');

/**
 * Set up real-time search as user types
 * The search is triggered on every input change in the search box
 */
searchBox.addEventListener('input', handleInputChange);

/**
 * Handles the search functionality
 * - Takes the query from the input box
 * - Searches using the WASM engine
 * - Displays only the first 5 results to avoid overwhelming the UI
 * - Shows results in a nicely formatted box with styling
 * 
 * @param {Event} event - The input event from the search box
 */
async function handleInputChange(event) {
    // Get the query string from the input box
    const query = event.target.value;

    // Call the Wasm search function
    const searchResults = pizza.search_by_query_string(query);

    // Extract only total_hits and hits fields, and limit hits to first 5 documents
    const { total_hits, hits } = searchResults;
    const limitedHits = hits.slice(0, 5);
    const filteredResults = { total_hits, hits: limitedHits };
    const jsonString = JSON.stringify(filteredResults, null, 2);

    // Display the results with enhanced styling
    resultsElement.innerText = jsonString;
    
    // Styling for the results box:
    // - Light gray border with rounded corners
    // - Comfortable padding and width
    // - Scrollable if content exceeds max height
    // - Subtle shadow for depth
    // - Centered on the page
    resultsElement.style.border = '1.5px solid #ccc';
    resultsElement.style.padding = '22.5px'; // 15px * 1.5
    resultsElement.style.borderRadius = '7.5px'; // 5px * 1.5
    resultsElement.style.backgroundColor = '#f8f8f8';
    resultsElement.style.width = '1200px'; // 800px * 1.5
    resultsElement.style.maxHeight = '750px'; // 500px * 1.5
    resultsElement.style.overflowY = 'auto';
    resultsElement.style.margin = '30px auto'; // 20px * 1.5
    resultsElement.style.boxShadow = '0 3px 6px rgba(0,0,0,0.1)'; // 2px * 1.5
}

/**
 * Loads and indexes a JSON file from a given URL
 * This function is used to initialize the search engine with data
 * 
 * @param {string} url - The URL of the JSON file to load
 */
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

        // Load the JSON data into the search engine
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

// Initialize the search engine with the default index file
const fileUrl = '/assets/index.json';
loadFileAndIndex(fileUrl);

