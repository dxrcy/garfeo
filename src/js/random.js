// Set href of random link to a random comic, on page load
function set_random_url(base_url, first_index, last_index) {
    // Filled at compile time
    // String is 4 digits, and JS parses as non-base10 without stringify
    let min = parseInt(first_index);
    let max = parseInt(last_index);
    // Get random number index
    let random = Math.floor(Math.random() * (max - min + 1) + min).toString();
    // Add zeros to start
    let index = "0".repeat(4 - random.length) + random;
    // Set link href
    let url = base_url + index;
    document.querySelector("#random").href = url;
}
