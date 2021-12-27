window.onload = function() {
    const themeToggle = document.querySelector(".theme-switch input[type='checkbox']");
    themeToggle.addEventListener("change", switchTheme, false);

    const textSubmit = document.getElementById("summarize");
    textSubmit.addEventListener("click", summarize, false);

    const calcSubmit = document.getElementById("calculate");
    calcSubmit.addEventListener("click", calculate, false);

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

async function summarize() {
    if (document.getElementById("text_input").value.length === 0) {
        return;
    }

    let loading = document.getElementById("loading");
    loading.style.display = "inline";

    let data = get_data(
        document.getElementById("text_input").value,
        "",
    );
    document.getElementById("text_output").value = await post(this.id, data);

    setSize();
    enable_hideable();
    loading.style.display = "none";
}

async function calculate() {
    if (document.getElementById("text_input").value.length === 0 ||
        document.getElementById("text_output").value.length === 0) {
        return;
    }

    let loading = document.getElementById("loading");
    loading.style.display = "inline";

    let data = get_data(
        document.getElementById("text_input").value,
        document.getElementById("text_output").value,
    );
    let resp = await post(this.id, data);

    setBleu(resp);
    setFreq();
    loading.style.display = "none";
}

function reset() {
    document.getElementById("text_input").value = "";
    document.getElementById("text_output").value = "";
    document.getElementById("source_count").value = "0";
    document.getElementById("abstract_count").value = "0";
    document.getElementById("reduction_percent").value = "0";
    disable_hideable();
}

function enable_hideable() {
    document.getElementById("calculate").classList.remove("disabled");
    document.getElementById("bleu").classList.remove("disabled");
}

function disable_hideable() {
    document.getElementById("calculate").classList.add("disabled");
    document.getElementById("bleu").classList.add("disabled");
}

function get_data(src, abs) {
    return JSON.stringify({
        src: src,
        abs: abs,
    });
}

async function post(endpoint, data) {
    let resp = await fetch(
        `http://127.0.0.1:51440/${endpoint}`,
        {
            method: "POST",
            headers: {
                "content-type": "application/json",
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

function setBleu(bleu) {
    console.log(bleu);
}

function setFreq() {

}
