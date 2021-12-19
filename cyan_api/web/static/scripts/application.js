window.onload = function() {
    const themeToggle = document.querySelector(".theme-switch input[type='checkbox']");
    themeToggle.addEventListener("change", switchTheme, false);

    const textInput = document.getElementById("submit");
    textInput.addEventListener("click", submit, false);

    const textReset = document.getElementById("reset");
    textReset.addEventListener("click", reset, false);
}

function switchTheme(e) {
    if (e.target.checked) {
        // Theme => Dark Mode
        document.documentElement.setAttribute("theme", "dark");
        document.getElementById("icon_sun").style.display = "block";
        document.getElementById("icon_moon").style.display = "none";
    }
    else {
        // Theme => Light Mode
        document.documentElement.setAttribute("theme", "light");
        document.getElementById("icon_sun").style.display = "none";
        document.getElementById("icon_moon").style.display = "block";
    }
}

async function submit() {
    let loading = document.getElementById("loading");
    loading.style.display = "inline";
    await sleep(5000);

    loading.style.display = "none";
}

function reset() {
    document.getElementById("text_input").value = "Enter text to summarize.";
    document.getElementById("text_output").value = "";
}

function sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}
