// Set href of random link to a random comic, on page load
function set_random_url() {
    // Get random number index
    let random = Math.floor(Math.random() * (LAST_INDEX + 1)).toString();
    // Add zeros to start
    let index = "0".repeat(4 - random.length) + random;
    // Set link href
    let url = BASE_URL + index;
    document.querySelector("#random").href = url;

    register_navigation_key("r", index);
}
