window.onload = function() {
    const themeToggle = document.querySelector(".theme-switch input[type='checkbox']");
    themeToggle.addEventListener("change", switchTheme, false);

    const textSubmit = document.getElementById("summarize");
    textSubmit.addEventListener("click", summarize, false);

    const setNGram = document.getElementById("n_gram");
    setNGram.addEventListener("change", calculate, false);

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

    document.getElementById("loading").style.display = "inline";
    await post("summarize");
    document.getElementById("loading").style.display = "none";
}

async function calculate() {
    if (document.getElementById("text_input").value.length === 0 ||
        document.getElementById("text_output").value.length === 0) {
        return;
    }

    await post("calculate");
}

function reset() {
    document.getElementById("text_input").value = "";
    document.getElementById("text_output").value = "";
    document.getElementById("source_count").value = "0";
    document.getElementById("abstract_count").value = "0";
    document.getElementById("reduction_percent").value = "0";
    hide_disabled();
}

async function get(endoint) {
    await fetch(
        `http://127.0.0.1:51440/${endpoint}`,
        {
            method: "GET",
        }
    );
}

async function post(endpoint) {
    let data = getData();
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
    setFields(String(await resp.text()));
    view_disabled();
}

function getData() {
    return JSON.stringify({
        src: document.getElementById("text_input").value,
        abs: document.getElementById("text_output").value,
        n: document.getElementById("n_gram").value,
    });
}

function setFields(resp) {
    // Set abstraction text (output)
    let data = JSON.parse(resp);
    if (data.abs.length > 0) {
        document.getElementById("text_output").value = data.abs;
    }

    // Set reduction values
    let src = document.getElementById("text_input").value.length;
    let abs = document.getElementById("text_output").value.length;
    let red = src === 0 ? 0 : (100 * (src - abs)) / src;
    document.getElementById("source_count").textContent = src;
    document.getElementById("abstract_count").textContent = String(abs);
    document.getElementById("reduction_percent").textContent = red.toFixed(2).toString();

    // Set BLEU % values
    document.getElementById("bleu_score").textContent = data.bleu.score;

    // Set N-Gram Comparison Values
    document.getElementById("source_n_gram").src = "../static/img/ng_src.png?" + Math.random();
    document.getElementById("abstract_n_gram").src = "../static/img/ng_abs.png?" + Math.random();

    // Set Frequency Values
    document.getElementById("pos_frequency").src = "../static/img/freq.png?" + Math.random();
}

function view_disabled() {
    document.getElementById("bleu").classList.remove("disabled");
    document.getElementById("top_n_grams").classList.remove("disabled");
    document.getElementById("freq").classList.remove("disabled");
}

function hide_disabled() {
    document.getElementById("bleu").classList.add("disabled");
    document.getElementById("top_n_grams").classList.add("disabled");
    document.getElementById("freq").classList.add("disabled");
}
