// Go to previous/next post by index
function register_navigation_key(key, index) {
    window.addEventListener("keydown", (event) => {
        if (event.altKey || event.ctrlKey) {
            return;
        }
        if (event.key == key) {
            location.href = BASE_URL + index;
        }
    });
}

