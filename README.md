# OSSINTSS2

**osintss2** je vysoko výkonná, modulárna platforma navrhnutá pre sofistikované zhromažďovanie informácií a hĺbkovú analýzu zo širokého spektra otvorených zdrojov. Je postavená na jazyku Rust, pričom prioritou je spoľahlivosť, rýchlosť a rozšíriteľnosť pre náročné dátové operácie.

Systém umožňuje komplexný zber dát prostredníctvom zásuvnej architektúry, ktorá dovoľuje integráciu rozmanitých dátových zdrojov. Zhromaždené informácie môžu byť ukladané a spracovávané pomocou integrovaných dokumentovo orientovaných a grafových databázových backendov, čo umožňuje rozsiahlu správu dát aj komplexnú relačnú analýzu.

## Kľúčové Vlastnosti

* **Modulárna Architektúra Zberačov (`registries`):** Ľahko rozšíriteľná o nové moduly pre prístup k rôznym kategóriám otvorených informácií, vrátane, ale nie výlučne:
    * Webový obsah a archívy
    * Platformy sociálnych médií
    * Firemné a právne záznamy
    * Sieťová inteligencia (DNS, WHOIS, IP)
    * Feedy hrozbovej inteligencie (Threat intelligence)
    * Repozitáre únikov dát
    * Špecializované verejné dátové sady
* **Pokročilá Perzistencia Dát (`db`):**
    * **Dokumentové Úložisko (Elasticsearch):** Optimalizované pre indexovanie, vyhľadávanie a analýzu veľkých objemov štruktúrovaných a neštruktúrovaných textových dát.
    * **Grafové Úložisko (Neo4j):** Umožňuje výkonné modelovanie a dopytovanie vzťahov a sietí medzi entitami, čo je kľúčové pre odhaľovanie nečakaných prepojení.
* **Robustná Správa API Kľúčov (`apikey_broker`):** Centralizované a bezpečné zaobchádzanie s API prihlasovacími údajmi pre prístup k externým službám.
* **Všestranný Export Dát (`export`):** Uľahčuje extrakciu spracovaných výsledkov v rôznych formátoch (napr. CSV) pre ďalšie použitie alebo reporting.
* **Efektívne Rozhranie Príkazového Riadku:** Poskytuje výkonné a skriptovateľné rozhranie pre orchestráciu úloh zberu a správu platformy, využívajúc `clap` pre parsovanie argumentov a `rustyline` pre vylepšený interaktívny zážitok.
* **Rámec Pripravený na AI (`ai`):** Navrhnutý s rozhraním pre integráciu schopností umelej inteligencie (AI) a strojového učenia (ML) na rozšírenie spracovania dát a analytických pracovných postupov (napr. spracovanie prirodzeného jazyka, rozpoznávanie vzorov).
* **Výkon a Spoľahlivosť:** Vyvinuté v jazyku Rust, čo zaisťuje bezpečnosť pamäte, súbežnosť a vysokorýchlostné vykonávanie pre intenzívne OSINT operácie. Asynchrónne operácie prostredníctvom `tokio` sú rozsiahlo využívané.
* **Riadené Konfiguráciou (`config.rs`, `config.yml`):** Správanie systému, pripojenia k databázam a parametre modulov sú spravované prostredníctvom centrálneho konfiguračného systému.

## Cieľové Prostredie

Táto platforma je určená pre profesionálov a organizácie vyžadujúce robustný a prispôsobiteľný systém na vykonávanie podrobných vyšetrovaní, koreláciu komplexných dátových súborov a podporu strategického získavania informácií. Je navrhnutá na spracovanie značných objemov dát a zložitých analytických úloh.

## Predpoklady

* Rust Toolchain (odporúčaná najnovšia stabilná verzia)
* Docker (alebo priame inštalácie Elasticsearch a Neo4j)
* Bežiaca inštancia Elasticsearch
* Bežiaca inštancia Neo4j

## Nastavenie a Konfigurácia

### 1. Nastavenie Databáz

Uistite sa, že vaše inštancie Elasticsearch a Neo4j sú spustené a dostupné.

**Elasticsearch:**
* Zvyčajne beží na `http://localhost:9200`.
* Aplikácia zvyčajne nevyžaduje žiadnu špecifickú predkonfiguráciu indexov, mala by si ich v prípade potreby vytvárať dynamicky.

**Neo4j:**
* Zvyčajne beží na `bolt://localhost:7687` s webovým rozhraním na `http://localhost:7474`.
* Uistite sa, že máte používateľské prihlasovacie údaje (používateľské meno/heslo) pre inštanciu Neo4j.

### 2. Konfigurácia Aplikácie

Primárna konfigurácia je spravovaná prostredníctvom súboru `config.yml` (alebo podobného, ako je definované v `src/config.rs`). Tento súbor je potrebné vytvoriť a naplniť potrebnými parametrami, vrátane:

* Detailov pripojenia k databázam (URL, používateľské mená, heslá pre Elasticsearch a Neo4j).
* API kľúčov pre rôzne moduly v `registries`. Tieto by mali byť spravované prostredníctvom systému `apikey_broker` podľa jeho konfiguračnej sekcie.
    * Pre očakávanú štruktúru si pozrite `src/apikey_broker.rs` a `src/config.rs`.
* Ostatných prevádzkových parametrov.

Môže byť poskytnutá šablóna alebo príklad `config.yml`, alebo si ho budete musieť vytvoriť na základe štruktúry `Settings` v `src/config.rs`.

```yaml
# Príklad štruktúry config.yml (prispôsobte podľa src/config.rs)
elasticsearch:
  url: "http://localhost:9200"
  # user: "elastic" # ak je zapnuté zabezpečenie
  # password: "changeme" # ak je zapnuté zabezpečenie

neo4j:
  uri: "bolt://localhost:7687"
  user: "neo4j"
  password: "vashe_neo4j_heslo" # Zmeňte toto!

# api_keys:
#   google_cse_id: "vas_google_cse_id"
#   google_api_key: "vas_google_api_key"
#   # ... ostatné API kľúče podľa požiadaviek modulov registrov
```

### 3. Kompilácia Aplikácie

Prejdite do koreňového adresára projektu a skompilujte aplikáciu pomocou Cargo:

```bash
cargo build --release
```
Spustiteľný súbor sa bude nachádzať v `target/release/osintss2` (upravte podľa názvu binárneho súboru definovaného vo vašom `Cargo.toml`, pravdepodobne `osintss` alebo `osintss2.0`).

## Základné Použitie

Aplikácia sa ovláda prostredníctvom rozhrania príkazového riadku. Presné príkazy a podpríkazy sú definované pomocou knižnice `clap` v súbore `src/main.rs`.

Pre zobrazenie dostupných príkazov a možností spustite (nahraďte `osintss2` skutočným názvom vášho spustiteľného súboru):
```bash
./target/release/osintss2 --help
```

Všeobecný pracovný postup môže zahŕňať:
1.  **Konfiguráciu** systému a API kľúčov.
2.  **Iniciovanie úloh zberu dát** pomocou špecifických modulov registrov alebo hlavného príkazu pre zber.
3.  **Dopytovanie a analýzu** uložených dát v Elasticsearch alebo Neo4j priamo alebo prostredníctvom nástrojov poskytovaných platformou.
4.  **Exportovanie** výsledkov podľa potreby.

*(Ďalšie podrobnosti o špecifických príkazoch a operačných postupoch by mali byť udržiavané v internej dokumentácii.)*

## Prehľad Modulov

* **`src/main.rs`**: Hlavný vstupný bod aplikácie a parsovanie argumentov CLI.
* **`src/config.rs`**: Definuje konfiguračné štruktúry a mechanizmy načítania.
* **`src/registries/`**: Obsahuje jednotlivé moduly pre zber dát z rôznych zdrojov. Každý modul je zodpovedný za načítanie a predspracovanie dát zo svojho prideleného zdroja.
* **`src/db/`**: Spravuje pripojenia a interakcie s Elasticsearch (`document_db.rs`) a Neo4j (`graph_db.rs`).
* **`src/ai/mod.rs`**: Placeholder/Rozhranie pre integráciu úloh spracovania AI/ML.
* **`src/export.rs`**: Zabezpečuje funkcionality exportu dát.
* **`src/apikey_broker.rs`**: Spravuje bezpečné získavanie a používanie API kľúčov.
* **`src/utils.rs`**: Bežné pomocné funkcie používané naprieč projektom.
* **`src/setup.rs`**: Potenciálne rieši počiatočné nastavenie alebo validačné úlohy.

## Poznámky pre Vývojárov

* **Pridávanie Nových Zberačov:** Pre pridanie nového dátového zdroja vytvorte nový Rust modul v adresári `src/registries/`, implementujte požadované traity (pravdepodobne `AsyncCollector` alebo podobný, ktorý bude definovaný rozhraniami projektu) a zaregistrujte ho v `src/registries/mod.rs` a potenciálne v hlavnom dispečerovi príkazov.
* **Spracovanie API Kľúčov:** Zabezpečte, aby sa nové moduly vyžadujúce API kľúče integrovali so systémom `apikey_broker`.
* **Spracovanie Chýb:** Využívajte knižnicu `anyhow` pre robustnú správu chýb.
* **Asynchrónne Operácie:** Využívajte `tokio` a `async/await` pre neblokujúce I/O.
* **Testovanie:** Komplexné jednotkové a integračné testy sú kľúčové pre udržanie stability a spoľahlivosti.

## Dôležité Upozornenia

* Táto platforma je navrhnutá pre profesionálne použitie oprávneným personálom.
* Používatelia sú výhradne zodpovední za zabezpečenie toho, aby všetky operácie vykonávané pomocou tejto platformy boli v súlade so všetkými platnými miestnymi, národnými a medzinárodnými zákonmi, nariadeniami a etickými usmerneniami.
* So zhromaždenými dátami by sa malo zaobchádzať s primeranými bezpečnostnými opatreniami a v súlade s predpismi o ochrane osobných údajov.
```
