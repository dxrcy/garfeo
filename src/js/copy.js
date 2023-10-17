// Highlight element text with client cursor
function select(element) {
    var range = document.createRange();
    range.selectNodeContents(element);
    var selection = window.getSelection();
    selection.removeAllRanges();
    selection.addRange(range);
}

// Copy element text to clipboard
function copy(element) {
  navigator.clipboard.writeText(element.innerText)
      .catch(function(err) {
        console.error("Failed to copy text: ", err);
      });
} 
