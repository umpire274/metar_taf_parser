# AGENTS.md

## Scopo

Questo file definisce le istruzioni operative che un agente AI deve seguire quando analizza, modifica, genera o
rifattorizza codice in questo repository.

L'obiettivo è mantenere il progetto coerente, manutenibile, prevedibile e allineato alle convenzioni tecniche già
adottate.

---

## Linguaggio principale del codice

* Tutto il codice sorgente deve essere scritto in **Rust**, salvo diversa richiesta esplicita.
* Va privilegiato uno stile **idiomatico Rust**, evitando trasposizioni meccaniche da altri linguaggi.
* Il codice generato deve rispettare le buone pratiche moderne di Rust:

    * gestione chiara di ownership e borrowing
    * uso di tipi forti
    * gestione esplicita degli errori
    * organizzazione modulare
    * riduzione al minimo delle clonazioni non necessarie
    * evitare panic non giustificati

---

## Linguaggio naturale delle spiegazioni

* Tutte le spiegazioni rivolte al proprietario del repository devono essere scritte in **italiano**.

* Devono essere in italiano anche:

    * spiegazioni tecniche
    * descrizioni delle modifiche
    * note di review
    * motivazioni delle scelte implementative
    * testi di commit e pull request, se richiesti

* Fanno eccezione la documentazione RustDoc e i commenti tecnici nel codice, che devono essere in **inglese** secondo le
  regole riportate sotto.

---

## Regole di documentazione

### Obbligo di RustDoc

Ogni nuovo elemento pubblico aggiunto al codice deve essere documentato con **RustDoc in lingua inglese**.

Questo vale, quando applicabile, per:

* moduli
* struct
* enum
* trait
* funzioni
* metodi
* costanti
* type alias

### Linee guida RustDoc

I commenti RustDoc devono:

* essere scritti in **inglese**
* spiegare lo scopo dell'elemento, non limitarsi a ripeterne il nome
* descrivere parametri, valori restituiti ed errori quando utile
* includere esempi per le API non banali
* chiarire invarianti, vincoli, effetti collaterali e assunzioni di sicurezza quando rilevanti

Esempio:

```rust
/// Parses a METAR visibility token and returns normalized visibility data.
///
/// This function supports both standard visibility groups and directional
/// minimum visibility groups when present.
///
/// # Arguments
///
/// * `token` - The raw METAR token to parse.
///
/// # Returns
///
/// Returns `Some(VisibilityInfo)` if the token is valid, otherwise `None`.
fn parse_visibility(token: &str) -> Option<VisibilityInfo> {
    // ...
}
```

---

## Commenti nel codice

* Va preferito codice chiaro e autoesplicativo rispetto a un eccesso di commenti inline.

* I commenti inline devono essere aggiunti solo quando apportano reale valore, ad esempio per:

    * spiegare logiche non immediate
    * documentare edge case
    * chiarire regole di business
    * motivare una scelta tecnica non ovvia

* I commenti tecnici nel codice Rust devono essere scritti in **inglese**.

* Devono essere evitati commenti banali o ridondanti.

Esempi da evitare:

* `// increment counter`
* `// create vector`

---

## Gestione degli errori

* Va preferito `Result<T, E>` ai panic.
* In codice di produzione bisogna evitare `.unwrap()` e `.expect()`, salvo casi eccezionali e ben motivati.
* Quando inevitabili, il motivo deve essere chiaro dal contesto o documentato.
* I messaggi di errore devono essere chiari, specifici e utili.

Pattern preferiti:

* `thiserror` per errori strutturati
* `anyhow` dove appropriato a livello applicativo
* propagazione degli errori con `?`
* aggiunta di contesto agli errori quando utile

I principali tipi di errore sono:

- `MetarError` (`src/metar/errors.rs`)
- `TafError` (`src/taf/errors.rs`)

Questi tipi sono usati come error type nei parser pubblici e sono serializzabili/testabili.

---

## Stile del codice

Il codice generato deve privilegiare:

* leggibilità
* funzioni piccole e focalizzate
* nomi significativi
* basso livello di annidamento
* separazione chiara delle responsabilità
* minima duplicazione
* moduli coesi

Preferire:

* `enum` al posto di stringhe magiche
* strutture tipizzate al posto di tuple poco leggibili, quando migliora la chiarezza
* iteratori quando rendono il codice più chiaro
* `Path` e `PathBuf` per operazioni sul filesystem
* uso idiomatico di `Option` e `Result`

Evitare:

* codice inutilmente complesso
* astrazioni premature
* generalizzazioni non necessarie
* funzioni monolitiche
* effetti collaterali nascosti

### Pattern tipico di parser

Tutte le funzioni pubbliche di parsing seguono la firma:

```rust
fn parse_metar(input: &str) -> Result<Metar, MetarError>
fn parse_taf(input: &str) -> Result<Taf, TafError>
```

I modelli di dominio sono fortemente tipizzati (`Metar`, `Taf`, ecc.) e serializzabili con `serde`.

---

## Organizzazione del progetto

Quando viene aggiunto nuovo codice:

* rispettare la struttura dei moduli esistente
* evitare di concentrare troppa logica in `main.rs`
* estrarre la logica riutilizzabile in moduli dedicati
* mantenere separate, per quanto possibile:

    * logica di dominio
    * parsing
    * formattazione
    * persistenza
    * utility

> **Nota:** Questo progetto espone solo una libreria (`lib.rs`), non una CLI. La separazione CLI non si applica attualmente.

La struttura dei moduli segue il pattern:

```
src/
  airports/
    db.rs, model.rs, mod.rs
  common/
    report_modifier.rs, tokenizer.rs, ...
  metar/
    models/
      metar.rs, wind.rs, ...
    parser/
      metar.rs, wind.rs, ...
  taf/
    models/
    parser/
```

Ogni gruppo parser/metar/taf ha sottodirectory `models` e `parser` per separare i modelli di dominio dalla logica di parsing.

---

## Aspettative sui test

Quando viene introdotta nuova logica, devono essere considerati anche i test.

Sono particolarmente desiderabili:

* unit test per la logica pura
* test sui casi limite dei parser
* regression test per bug già corretti
* integration test quando il comportamento coinvolge più componenti

Se una modifica non include test, il motivo va esplicitato.

### Pipeline di build e test

La pipeline di build e QA raccomandata è tramite gli script condivisi in `dev_tools/`:

- Windows: `dev_tools/build_check.ps1`
- Linux/macOS: `dev_tools/build_check.sh`

Questi script eseguono build, lint, format e test in modo standardizzato. Lanciare sempre la pipeline prima di push.

Per eseguire solo i test:

- `cargo test`

---

## Compatibilità e dipendenze

* Evitare di introdurre nuove dipendenze senza una necessità concreta.
* Ogni nuova crate deve essere coerente con lo stile e i bisogni del progetto.
* Privilegiare dipendenze:

    * mature
    * ben mantenute
    * largamente adottate
    * ben documentate

Quando possibile:

* preferire la standard library
* evitare crate pesanti per esigenze semplici
* ridurre il rischio di lock-in architetturale

Le uniche dipendenze esterne attualmente ammesse sono:

- `thiserror` (errori tipizzati)
- `serde` (serializzazione)

L'aggiunta di nuove dipendenze è fortemente scoraggiata salvo reale necessità e va sempre motivata.

---

## Performance e semplicità

* Ottimizzare solo quando serve davvero o quando il beneficio è evidente.
* Non sacrificare la leggibilità per micro-ottimizzazioni premature.
* In presenza di codice critico per prestazioni, mantenere comunque chiarezza e documentazione.
* Se una scelta implementativa è guidata dalla performance, esplicitarlo chiaramente.

---

## Refactoring

Quando viene proposto un refactoring, questo deve puntare a:

* migliorare leggibilità
* ridurre duplicazione
* isolare responsabilità
* facilitare manutenzione e test
* preservare il comportamento esistente, salvo richiesta diversa

Evitare refactoring puramente estetici se non producono un vantaggio concreto.

---

## Policy di porting e regex

Tutti i parser di gruppo (METAR/TAF) devono mantenere la parità di comportamento con i regex Python di riferimento, come descritto in `docs/PORTING_REGEX_POLICY.md`.

La baseline di porting è completa dalla versione `v0.3.0`. Ogni modifica ai parser deve preservare la compatibilità con i test di parità e la semantica dei gruppi.

### Regex Python → parsing manuale Rust

Il progetto Python di riferimento ([umpire274/python-metar-taf-parser](https://github.com/umpire274/python-metar-taf-parser)) implementa ogni gruppo parser tramite classi `Command` con **regex compilate** (`re.compile()`), ad esempio:

```python
class WindCommand:
    regex = r'^(VRB|\d{3})(\d{2})G?(\d{2,3})?(KT|MPS|KM\/H)?'
```

Il porting Rust ha **deliberatamente sostituito le regex con parsing manuale** idiomatico:
- `strip_prefix` / `strip_suffix` per riconoscere suffissi e prefissi
- slicing diretto (`&token[0..3]`)
- `chars().all(|c| c.is_ascii_digit())` per validazione carattere per carattere
- `split_once` per separare parti del token
- `parse::<u16>()` per conversione numerica

**Non è usato nessun crate regex nel progetto Rust.** La semantica di accettazione/rifiuto dei token è preservata rispetto al Python; solo l'implementazione è diversa. Questo è intenzionale e idiomatico Rust.

---

## Gap di parità rispetto al Python

Il progetto Python include funzionalità **non ancora portate** nel porting Rust v0.3.0. Un agente che lavora su questo repository deve essere consapevole di queste aree prima di proporre estensioni:

### 1. Conversione in linguaggio naturale (i18n)

Il Python espone un sistema i18n completo basato su `gettext` con file `messages.po/messages.mo` per **10 lingue**:
`de`, `en`, `es`, `fr`, `it`, `pl`, `ru-RU`, `tr`, `zh-CN`

Il sistema traduce ogni valore dei modelli in stringhe human-readable tramite chiavi come:
- `Phenomenon.RA` → `"rain"`
- `CloudQuantity.BKN` → `"broken"`
- `Flag.AUTO` → `"automated METAR"`
- `Remark.PeakWind` → `"peak wind of {1} knots from {0} degrees at {2}:{3}"`

Nel Rust v0.3.0 **non esiste nessun modulo di conversione in linguaggio naturale né alcun supporto i18n.**

### 2. Remark parser

Il Python include un `RemarkParser` dedicato con decine di pattern localizzati (gruppi `Remark.*` nel `.pot`) che converte i gruppi RMK in stringhe descrittive leggibili. Il Rust gestisce i RMK come stringa grezza (`rmk: Option<String>`).

### 3. Architettura Command/Supplier Python

Il Python usa un pattern architetturale `Command/Supplier` con classi dedicate per ogni gruppo (`WindCommand`, `CloudCommand`, ecc.) e un `CommandSupplier` per il dispatch. Il Rust usa invece funzioni pure di parsing, senza questo strato di indirection.

### 4. Modelli Python più ricchi

Il Python modella alcuni concetti non ancora presenti nel Rust, tra cui:
- `Icing` e `Turbulence` (con intensità e altezze) nei TAF trend
- `RunwayInfo` con `deposit_type`, `coverage`, `thickness`, `braking_capacity`
- variazione direzionale del vento (`min_variation`, `max_variation` sul `Wind`)
- `nosig` come campo booleano su `Metar`

Queste funzionalità sono candidate a porting incrementale futuro.

---

## Output atteso dall'agente

Quando l'agente propone nuovo codice o una modifica significativa, è preferibile fornire:

* spiegazione in italiano
* codice in Rust
* RustDoc in inglese per gli elementi pubblici
* eventuali commenti tecnici nel codice in inglese
* attenzione a coerenza stilistica, compilabilità e integrazione nel progetto

Quando utile, includere anche:

* motivazione della scelta tecnica
* possibili alternative
* impatti sui moduli esistenti
* suggerimenti per test o refactoring correlati

---

## Convenzioni di review

Durante review o proposta di modifica:

* evidenziare prima i problemi reali
* distinguere chiaramente tra:

    * errori
    * rischi
    * miglioramenti consigliati
    * preferenze stilistiche

Le osservazioni devono essere:

* tecniche
* concrete
* motivate
* orientate alla soluzione

---

## Convenzioni per commit e pull request

Se vengono suggeriti messaggi di commit o testi per pull request:

* scriverli in **italiano**, salvo diversa richiesta
* mantenerli chiari e descrittivi
* evitare messaggi troppo generici come:

    * `fix`
    * `update`
    * `various changes`

Esempi migliori:

* `Refactor parser METAR per gestire gruppi di visibilità multipli`
* `Aggiunge supporto al logging su file con opzione --log`
* `Corregge il calcolo dell'ETA nella barra di avanzamento`

---

## Indicazioni operative per l'agente AI

Quando proponi modifiche:

* non riscrivere interi file se non necessario
* preferisci patch piccole e mirate
* preserva nomi, struttura e stile già presenti nel repository
* prima di introdurre nuove astrazioni, verifica che siano davvero utili
* se individui un bug, spiega il problema in italiano prima di proporre la correzione
* se una modifica può rompere la compatibilità, segnalalo esplicitamente
* se mancano informazioni, fai assunzioni conservative e dichiarale chiaramente
* proponi sempre soluzioni realistiche per un progetto Rust reale e manutenibile

---

## Priorità generali

In caso di dubbio, dare priorità a:

1. correttezza
2. chiarezza
3. manutenibilità
4. coerenza con il progetto
5. performance

---

## Istruzione finale

Quando generi o modifichi codice per questo repository:

* usa **Rust** come linguaggio principale
* scrivi le **spiegazioni in italiano**
* documenta i nuovi elementi pubblici con **RustDoc in inglese**
* usa commenti tecnici nel codice solo quando servono davvero, anch'essi in **inglese**
* proponi soluzioni coerenti con un progetto Rust reale, curato e manutenibile
