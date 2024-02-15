function register_navigation_key(key, new_index) {
    window.addEventListener("keydown", (event) => {
        if (event.altKey || event.ctrlKey) {
            return;
        }
        if (event.key == key) {
            location.href = BASE_URL + new_index;
        }
    });
}

