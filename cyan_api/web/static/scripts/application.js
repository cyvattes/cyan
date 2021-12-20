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

    let data = document.getElementById("text_input").value;
    document.getElementById("text_output").value = await summarize(data);

    loading.style.display = "none";
}

function reset() {
    document.getElementById("text_input").value = "";
    document.getElementById("text_output").value = "";
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
