// Go to a post by index
window.addEventListener("keydown", (event) => {
    if (event.altKey || event.ctrlKey) {
        return;
    }
    if (event.key == "g") {
        // Use prompt input
        let input = prompt("Iru al bildstrio:");
        let path = getUrlPath(input);
        if (path == null) {
            return;
        }
        location.href = BASE_URL + path;
    }
});

function getUrlPath(input) {
    if (input == null || input == undefined || input.length === 0) {
        return null;
    }
    if (input[0].toLowerCase() === "h") { return ""; }
    if (input[0].toLowerCase() === "k") { return "krado"; }
    if (input[0].toLowerCase() === "l") { return "listo"; }

    // Parse index
    let index;
    if (input[0] == "-") {
        index = LAST_INDEX;
    } else {
        index = parseInt(input);
        if (isNaN(index) || index < 0 || index > LAST_INDEX) {
            return null;
        }
    }
    // Add zeros to start
    index = index.toString();
    return "0".repeat(4 - index.length) + index;
}

