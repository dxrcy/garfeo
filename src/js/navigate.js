function register_navigation_key(base_url, key, index) {
    window.addEventListener("keydown", (event) => {
        if (event.altKey || event.ctrlKey) {
            return;
        }
        if (event.key == key) {
            location.href = base_url + index;
        }
    });
}

