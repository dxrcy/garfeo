// Highlight element text with client cursor
function select(element) {
    var range = document.createRange();
    range.selectNodeContents(element);
    var selection = window.getSelection();
    selection.removeAllRanges();
    selection.addRange(range);
}

// Copy element text to clipboard
function copy_text(text) {
    navigator.clipboard.writeText(text)
        .catch(function (err) {
            console.error("Failed to copy text:", err);
        });
} 

// Copy post index
function register_copy_key(index) {
    window.addEventListener("keydown", (event) => {
        if (event.altKey || event.ctrlKey) {
            return;
        }
        if (event.key == "y") {
            copy_text(index);
        }
    });
}

