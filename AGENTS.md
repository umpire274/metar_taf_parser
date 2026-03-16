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

---

## Organizzazione del progetto

Quando viene aggiunto nuovo codice:

* rispettare la struttura dei moduli esistente
* evitare di concentrare troppa logica in `main.rs`
* estrarre la logica riutilizzabile in moduli dedicati
* mantenere separate, per quanto possibile:

    * CLI
    * logica di dominio
    * parsing
    * formattazione
    * persistenza
    * utility

Se una modifica supera la dimensione di una patch semplice, preferire:

* introduzione di un nuovo modulo
* refactoring in funzioni più piccole
* confini più chiari tra file e responsabilità

---

## Aspettative sui test

Quando viene introdotta nuova logica, devono essere considerati anche i test.

Sono particolarmente desiderabili:

* unit test per la logica pura
* test sui casi limite dei parser
* regression test per bug già corretti
* integration test quando il comportamento coinvolge più componenti

Se una modifica non include test, il motivo va esplicitato.

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
