# 💻OSSINTSS2🕵️

**Osintss2** je vysoko výkonná, modulárna platforma navrhnutá pre sofistikované zhromažďovanie informácií a hĺbkovú analýzu zo širokého spektra otvorených zdrojov. Je postavená na jazyku Rust ⚙️, pričom prioritou je spoľahlivosť, rýchlosť a rozšíriteľnosť pre náročné dátové operácie.

Systém umožňuje komplexný zber dát prostredníctvom zásuvnej architektúry 🧩, ktorá dovoľuje integráciu rozmanitých dátových zdrojov. Zhromaždené informácie môžu byť ukladané a spracovávané pomocou integrovaných dokumentovo orientovaných 📄 a grafových databázových backendov 🕸️, čo umožňuje rozsiahlu správu dát aj komplexnú relačnú analýzu.

## 🎯 Kľúčové Vlastnosti

* **Modulárna Architektúra Zberačov (`registries`)** 🧩: Ľahko rozšíriteľná o nové moduly pre prístup k rôznym kategóriám otvorených informácií, vrátane, ale nie výlučne:
    * 🌐 Webový obsah a archívy 📜
    * 👥 Platformy sociálnych médií 💬
    * 🏛️ Firemné a právne záznamy 📂
    * 📡 Sieťová inteligencia (DNS, WHOIS, IP) 🔗
    * 🛡️ Feedy hrozbovej inteligencie (Threat intelligence) 🚨
    * 🔓 Repozitáre únikov dát 🔥
    * 📊 Špecializované verejné dátové sady 📚
* **Pokročilá Perzistencia Dát (`db`)** 💾🗄️:
    * **Dokumentové Úložisko (Elasticsearch)** 📄🔍: Optimalizované pre indexovanie, vyhľadávanie a analýzu veľkých objemov textových dát.
    * **Grafové Úložisko (Neo4j)** 🕸️🔗: Umožňuje modelovanie a dopytovanie zložitých vzťahov a sietí medzi entitami.

## 🛠️ Kľúčové Komponenty Systému

* **`src/main.rs`** 🚀: Vstupný bod aplikácie, spracovanie príkazov a orchestrácia.
* **`src/registries/`** 📦: Moduly pre zber dát z rôznych OSINT zdrojov.
* **`src/db/`** 🗃️: Správa pripojení a operácií s databázami (Elasticsearch, Neo4j).
* **`src/config.rs`** ⚙️: Načítavanie a správa konfiguračných nastavení.
* **`src/export.rs`** 📤: Funkcionalita pre export dát.
* **`src/apikey_broker.rs`** 🔑: Spravuje bezpečné získavanie a používanie API kľúčov.
* **`src/utils.rs`** 🔧: Bežné pomocné funkcie používané naprieč projektom.
* **`src/setup.rs`** 🔩: Potenciálne rieši počiatočné nastavenie alebo validačné úlohy.

## 🧑‍💻 Poznámky pre Vývojárov 📝

* **Pridávanie Nových Zberačov** ➕🔌: Pre pridanie nového dátového zdroja vytvorte nový Rust modul v adresári `src/registries/`, implementujte požadované traity (pravdepodobne `AsyncCollector` alebo podobný, ktorý bude definovaný rozhraniami projektu) a zaregistrujte ho v `src/registries/mod.rs` a potenciálne v hlavnom dispečerovi príkazov.
* **Spracovanie API Kľúčov** 🔑🔒: Zabezpečte, aby sa nové moduly vyžadujúce API kľúče integrovali so systémom `apikey_broker`.
* **Spracovanie Chýb** 🐛✅: Využívajte knižnicu `anyhow` pre robustnú správu chýb.
* **Asynchrónne Operácie** ⚡⏳: Využívajte `tokio` a `async/await` pre neblokujúce I/O.
* **Testovanie** ✔️🔬: Komplexné jednotkové a integračné testy sú kľúčové pre udržanie stability a spoľahlivosti.

## ⚠️ Dôležité Upozornenia 🚨

* 👤 Táto platforma je navrhnutá pre profesionálne použitie oprávneným personálom. 💼
* ⚖️ Používatelia sú plne zodpovední za dodržiavanie všetkých platných zákonov a etických smerníc pri používaní tohto nástroja. 🛡️
* 🔒 Neoprávnené použitie alebo použitie na nelegálne aktivity je prísne zakázané.
