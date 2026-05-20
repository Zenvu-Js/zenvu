// Create a panel in Chrome DevTools
chrome.devtools.panels.create(
    "Zenvu",
    "icon.png",
    "panel.html",
    function(panel) {
        console.log("Zenvu.js DevTools Panel Created!");
    }
);
