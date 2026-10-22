# Hodinový skokan / Time Hopper

<p align="center">
  <img src="./assets/icon.svg" alt="Hodinový skokan logo" width="96" height="96" />
</p>

<p align="center">
  <strong>Kolik bude hodin za 17 hodin?</strong><br />
  Minimalistická tray utilita, která ti to řekne hned — bez kalkulačky, bez kalendáře, bez otravného okna.
</p>

<p align="center">
  <a href="https://github.com/h0nyik/casoskok/releases/latest">Stáhnout nejnovější release</a>
  ·
  <a href="#english">English</a>
  ·
  <a href="#čeština">Čeština</a>
</p>

---

## Čeština

### Co to je
**Hodinový skokan** je malá desktopová aplikace pro Windows, macOS i Linux. Žije v systémové liště (tray / menu bar), otevře se kliknutím jako kompaktní popover bez klasické title bar lišty a po kliknutí venku zase zmizí.

Ideální, když často řešíš: *„Za kolik hodin to bude?“* — třeba směny, pečení, deadliny, spánek nebo výpočet konce směny.

### Hlavní funkce
- **Okamžitý výpočet času** — zadej hodiny, stiskni Spočítat
- **Tray / menu bar režim** — minimální footprint, žádný zbytečný taskbar clutter
- **Bez title baru** — čisté, prémiové lime-green UI
- **Always on top** popover při otevření
- **Dark / light režim**
- **Automatické aktualizace** přes GitHub Releases
- **Spuštění po startu systému** (volitelné)
- **Single instance** — druhý start jen otevře existující okno
- **PWA** varianta pro prohlížeč a mobilní plochu

### Stažení
Nejnovější instalátory najdeš v [Releases](https://github.com/h0nyik/casoskok/releases/latest):

| Platforma | Balíček |
| --- | --- |
| Windows | `.msi` / `.exe` (NSIS) |
| macOS | `.dmg` (universal) |
| Linux | `.AppImage` / `.deb` |

### Použití
1. Nainstaluj aplikaci.
2. Ikona se objeví v systémové liště.
3. Klikni na ikonu → zadej počet hodin → **Spočítat**.
4. Výsledek uvidíš ve formátu: `Bude 18:43, úterý 28.4.2026` + náhodný český citát.
5. Escape nebo klik mimo okno appku schová zpět do lišty.

### Vývoj
```bash
npm install
npm run desktop:dev
```

Build instalátorů:
```bash
npm run desktop:build
```

---

## English

### What it is
**Time Hopper** (*Hodinový skokan*) is a tiny cross-platform tray utility. It lives in the system tray / menu bar, opens as a borderless popover, and stays out of your way until you need a quick “what time will it be in X hours?” answer.

### Key features
- Instant hour-offset time calculation
- Tray / menu bar first UX with minimal footprint
- Borderless, always-on-top popover UI
- Dark / light theme
- Auto-updates via GitHub Releases
- Optional launch at login
- Single-instance desktop app
- Optional PWA install for browsers and mobile home screens

### Download
Grab the latest builds from [Releases](https://github.com/h0nyik/casoskok/releases/latest).

### Develop
```bash
npm install
npm run desktop:dev
```

### Keywords / SEO
time calculator, hours from now, tray app, menu bar app, Windows tray utility, macOS menu bar widget, Linux AppImage, dark mode timer helper, shift end calculator, “what time will it be”, Hodinový skokan, Time Hopper, časová utilita, výpočet hodin, systémová lišta

---

## Licence
UNLICENSED / private project unless stated otherwise in future releases.
