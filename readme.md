# Mishell

**Mishell** este un shell interactiv scris în Rust, proiectat pentru flexibilitate și
extensibilitate. Oferă o abordare modernă a caracteristicilor clasice ale shell-urilor,
fiind în același timp o bază pentru învățare și experimentare în dezvoltarea de shell-uri.

---

## Funcționalități

### Funcționalități de bază

- **Pipe-uri și Redirecționări**

  Execută secvențe complexe de comenzi prin conectarea ieșirilor și intrărilor între comenzi
  sau redirecționarea fluxurilor către fișiere.
  Exemplu:

  ```bash
  ls | grep "test" > results.txt
  ```

- **Variabile de Mediu**
  Gestionează variabilele de mediu fără efort, cu suport pentru export, modificare și utilizare.
  Exemplu:

  ```bash
  export PATH=/usr/bin:$PATH
  echo $PATH
  ```

- **Gestionarea Semnalelor**

  Răspunde la semnalele sistemului precum `Ctrl+C` (terminare) și `Ctrl+Z` (suspendare).

- **Personalizarea Promptului**

  Creează o experiență personalizată prin modificarea promptului pentru a include elemente
  precum numele utilizatorului, numele gazdei sau directorul curent.

- **Fișier de Configurare și Alias-uri**

  Încarcă configurații personalizate și definește comenzi rapide utilizând un fișier extern (ex. `.mishellrc`).
  Exemplu:

  ```bash
  alias ll="ls -la"
  ```

- **Raportarea Erorilor**

  Oferă feedback clar pentru comenzi sau erori de sintaxă nevalide, cu stări de ieșire bine definite.

---

### Funcționalități Extra

- **Completare Automată (Tab Completion)**

  Autocomplete pentru comenzi și nume de fișiere pentru o experiență mai fluentă.

- **Expansiune de Comenzi**

  Suport extins pentru sintaxe avansate precum `$()` sau `` ` ` `` pentru executarea de comenzi în subshell.
  Exemplu:
  ```bash
  echo "Astăzi este $(date)"
  ```

---

## Utilizare

1. **Compilare și Rulare**

   Compilează Mishell folosind `cargo`:

   ```bash
   cargo build --release
   ./mishell
   ```

2. **Configurație**

   Creează un fișier `.mishellrc` în directorul tău principal pentru alias-uri și setări personalizate.

3. **Comenzi Suportate**

   - Pipe-uri și redirecționări: `|`, `>`, `<`
   - Gestionarea variabilelor de mediu: `export`, `$VAR`
   - Comenzi interne: `cd`, `exit`, `help`

4. **Scurtături**

   - `Ctrl+C`: Întrerupe comanda curentă.
   - `Ctrl+D`: Iese din shell.
   - `Tab`: Completează automat comenzile (dacă este activat).

---

## Plan de Dezvoltare

### Funcționalități Viitoare

- Implementarea funcționalităților de bază (ex. pipe-uri, redirecționări, variabile de mediu).
- Extinderea la funcționalități extra, precum completarea automată și expansiunea comenzilor.

---

## Contribuții

Contribuțiile sunt binevenite! Indiferent dacă rezolvi bug-uri, adaugi funcționalități sau îmbunătățești documentația, poți face un fork și trimite un pull request.

---

## Licență

Mishell este software open-source licențiat sub [Licența MIT](LICENSE).
