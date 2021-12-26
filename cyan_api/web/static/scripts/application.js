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
    if (document.getElementById("text_input").value.length === 0) {
        return;
    }

    let loading = document.getElementById("loading");
    loading.style.display = "inline";

    let data = document.getElementById("text_input").value;
    document.getElementById("text_output").value = await summarize(data);

    setSize();
    setBleu();
    setFreq();

    loading.style.display = "none";
}

function reset() {
    document.getElementById("text_input").value = "";
    document.getElementById("text_output").value = "";
    document.getElementById("source_count").value = "0";
    document.getElementById("abstract_count").value = "0";
    document.getElementById("reduction_percent").value = "0";
}

async function summarize(data) {
    let resp = await fetch(
        "http://127.0.0.1:51440/summarize",
        {
            method: "POST",
            headers: {
                "content-type": "text/plain;charset=UTF-8",
            },
            body: data,
        }
    );
    return String(await resp.text());
}

function setSize() {
    let src = document.getElementById("text_input").value.length;
    let abs = document.getElementById("text_output").value.length;
    let red = src === 0 ? 0 : (100 * abs) / src;
    document.getElementById("source_count").textContent = src;
    document.getElementById("abstract_count").textContent = abs;
    document.getElementById("reduction_percent").textContent = red.toFixed(2).toString();
}

function setBleu() {

}

function setFreq() {

}
