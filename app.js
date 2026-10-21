const form = document.querySelector("#time-form");
const input = document.querySelector("#hours-input");
const targetTime = document.querySelector("#target-time");
const resultCopy = document.querySelector("#result-copy");
const currentTime = document.querySelector("#current-time");
const themeToggle = document.querySelector("#theme-toggle");

const locale = "cs-CZ";
const timeFormatter = new Intl.DateTimeFormat(locale, {
  hour: "2-digit",
  minute: "2-digit",
});
const weekdayFormatter = new Intl.DateTimeFormat(locale, { weekday: "long" });
const quotes = [
  "Kdo šetří čas, ten si ho večer stejně rozlije do čaje.",
  "Hodiny tikají, ale knedlík v guláši zná pravdu hlouběji.",
  "Za každým zpožděním stojí malý trpaslík s kalendářem.",
  "Čas je jen tramvaj, která zapomněla, kde má konečnou.",
  "Když nevíš kolik bude, zeptej se kaktusu. Mlčí přesně.",
  "Budoucnost voní po limetě a lehce po nabíječce.",
  "Minuta sem, minuta tam, hlavně ať se nerozteče pátek.",
  "Kdo přičítá hodiny, ten komanduje slunce v domácích pantoflích.",
  "Vesmír je budík, který si omylem nastavil snooze na století.",
  "Čas letí, když ho honíš v županu s kalkulačkou.",
];

const savedTheme = localStorage.getItem("theme");
const prefersLight = window.matchMedia("(prefers-color-scheme: light)").matches;
const initialTheme = savedTheme ?? (prefersLight ? "light" : "dark");
let activeHours = null;
let activeQuote = "";

document.documentElement.dataset.theme = initialTheme;
themeToggle.setAttribute("aria-pressed", String(initialTheme === "light"));

function formatTime(date) {
  return timeFormatter.format(date);
}

function updateClock() {
  const now = new Date();
  currentTime.textContent = `Teď ${formatTime(now)}`;

  if (activeHours !== null) {
    showResult(activeHours, { refreshQuote: false });
  }
}

function calculateTarget(hours) {
  const now = new Date();
  return new Date(now.getTime() + hours * 60 * 60 * 1000);
}

function formatDate(date) {
  return `${weekdayFormatter.format(date)} ${date.getDate()}.${date.getMonth() + 1}.${date.getFullYear()}`;
}

function pickQuote() {
  const quote = quotes[Math.floor(Math.random() * quotes.length)];
  return quote === activeQuote && quotes.length > 1 ? pickQuote() : quote;
}

function showError(message) {
  activeHours = null;
  activeQuote = "";
  targetTime.textContent = "--:--";
  resultCopy.textContent = message;
  resultCopy.classList.add("is-error");
}

function showResult(hours, options = { refreshQuote: true }) {
  activeHours = hours;
  if (options.refreshQuote || !activeQuote) {
    activeQuote = pickQuote();
  }

  const target = calculateTarget(hours);
  targetTime.textContent = `Bude ${formatTime(target)}, ${formatDate(target)}`;
  resultCopy.textContent = activeQuote;
  resultCopy.classList.remove("is-error");
}

form.addEventListener("submit", (event) => {
  event.preventDefault();

  const hours = Number.parseFloat(input.value.replace(",", "."));

  if (!Number.isFinite(hours) || hours < 0) {
    showError("Zadej kladný počet hodin.");
    input.focus();
    return;
  }

  showResult(hours);
});

themeToggle.addEventListener("click", () => {
  const nextTheme = document.documentElement.dataset.theme === "dark" ? "light" : "dark";
  document.documentElement.dataset.theme = nextTheme;
  localStorage.setItem("theme", nextTheme);
  themeToggle.setAttribute("aria-pressed", String(nextTheme === "light"));
});

input.addEventListener("input", () => {
  activeHours = null;
  activeQuote = "";
  resultCopy.classList.remove("is-error");
});

window.addEventListener("load", () => {
  if ("serviceWorker" in navigator) {
    navigator.serviceWorker.register("./sw.js").catch(() => {
      // Desktop wrapper can run without service worker support.
    });
  }
});

updateClock();
setInterval(updateClock, 1000);
