# Hodinový skokan

Minimalistická PWA aplikace pro otázku: „Kolik bude hodin za X hodin?“

## Spuštění

```bash
npm install
npm start
```

Potom otevři `http://localhost:4173`.

Desktopové okno přes Tauri:

```bash
npm run desktop:dev
```

## Instalace na plochu

- Windows/macOS: otevři aplikaci v Chromu nebo Edge a použij instalaci aplikace v adresním řádku.
- iOS/iPadOS: otevři v Safari, zvol Sdílet a potom Přidat na plochu.
- Android: otevři v Chromu a zvol Přidat na plochu / Instalovat aplikaci.

## Jednoduchý instalátor pro počítač

Ano, půjde to. Nejlepší další krok je zabalit appku do Tauri:

- Windows: `.msi` nebo `.exe` instalátor.
- macOS: `.dmg` / `.app`.
- Linux: `.deb`, `.rpm` nebo `.AppImage`.

Tauri umí přibalit vše potřebné pro běžného uživatele, takže výsledkem je klasický instalátor bez ručního spouštění serveru. Zároveň umí nativní volbu „vždy navrchu“, kterou PWA/web neumí vynutit.

Vygenerování instalátoru:

```bash
npm install
npm run desktop:build
```

Konkrétně pro macOS:

```bash
npm run desktop:build:mac
```

Konkrétně pro Windows:

```bash
npm run desktop:build:windows
```

Výstupy budou v `src-tauri/target/release/bundle`. Na macOS typicky `.dmg`, na Windows `.msi`/`.exe`, na Linuxu podle dostupných systémových nástrojů například `.deb`, `.rpm` nebo `.AppImage`.

Pro vývojáře je potřeba mít nainstalovaný Rust a systémové Tauri závislosti. macOS balíčky se nejspolehlivěji staví na macOS, Windows balíčky na Windows. Pro běžného uživatele už výsledný instalátor nic z toho nevyžaduje.

Automatické sestavení všech desktopových instalátorů je připravené v GitHub Actions. Stačí pushnout tag ve tvaru `v0.1.0`, případně workflow spustit ručně:

```bash
git tag v0.1.0
git push origin v0.1.0
```

Workflow vytvoří draft release s balíčky pro Windows, macOS a Linux.

## Poznámka k „vždy navrchu“

PWA ani běžný web nesmí z bezpečnostních důvodů samy vynutit režim „vždy navrchu“. Desktopová Tauri verze už má `alwaysOnTop` zapnuté v konfiguraci okna.
