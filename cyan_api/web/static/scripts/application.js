window.onload = function() {
    const themeToggle = document.querySelector('.theme-switch input[type="checkbox"]');
    themeToggle.addEventListener('change', switchTheme, false);
}

function switchTheme(e) {
    if (e.target.checked) {
        // Theme => Dark Mode
        document.documentElement.setAttribute('theme', 'dark');
        document.getElementById("icon_sun").style.display = "block";
        document.getElementById("icon_moon").style.display = "none";
    }
    else {
        // Theme => Light Mode
        document.documentElement.setAttribute('theme', 'light');
        document.getElementById("icon_sun").style.display = "none";
        document.getElementById("icon_moon").style.display = "block";
    }
}
