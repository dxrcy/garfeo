// Go to a post by index
window.addEventListener("keydown", (event) => {
    if (event.altKey || event.ctrlKey) {
        return;
    }
    if (event.key == "g") {
        // Use prompt input
        let index = prompt("Iru al bildstrio:");
        if (index == null) {
            return;
        }
        // Validate
        if (index.startsWith("-")) {
            index = LAST_INDEX;
        } else {
            index = parseInt(index);
            if (isNaN(index) || index < 0 || index > LAST_INDEX) {
                return;
            }
        }
        // Add zeros to start
        index = index.toString();
        index = "0".repeat(4 - index.length) + index;
        location.href = BASE_URL + index;
    }
});

