# Live Football Tracker & Analytics
## Opis projekta

Ovaj projekat predstavlja mikroservisnu aplikaciju za praćenje fudbalskih utakmica, koja omogućava korisnicima pregled liga, utakmica i detalja mečeva, kao i analitiku i generisanje izveštaja. Sistem integriše eksterni sportski API kako bi pribavljao podatke o rasporedu utakmica, rezultatima i događajima (golovi, žuti i crveni kartoni), uz primenu keširanja i odgovarajućih modela skladištenja podataka.

Projekat je realizovan u programskom jeziku Rust i dizajniran je kao mikroservisna arhitektura. Arhitektura je modularna i proširiva, što omogućava njeno dalje unapređenje u okviru izrade diplomskog rada.

## Cilj projekta

Cilj projekta je razvoj sistema koji:

* omogućava pregled top evropskih fudbalskih liga i takmičenja,

* pruža detalje utakmica (rezultati, događaji, postave),

* omogućava analitiku sportskih podataka kroz grafikone,

* omogućava generisanje PDF izveštaja,

* koristi kombinaciju relacionih i NoSQL baza podataka,

* primenjuje keširanje radi optimizacije performansi i smanjenja broja poziva ka eksternom API-ju,

* implementira autentifikaciju i autorizaciju korisnika.

## Tehnologije

* Backend: Rust

* Frontend: Web aplikacija (Angular)

* Eksterni API: football-data.org ili API-Football

* Relacione baze: PostgreSQL

* NoSQL baza: MongoDB

* Keš: Redis

* Autentifikacija: JWT

* Analitika i grafici: Chart.js

* PDF generisanje: Rust PDF biblioteka (server-side)

## Arhitektura sistema

Sistem je implementiran kao mikroservisna aplikacija sa četiri servisa, gde svaki servis poseduje sopstvenu bazu podataka.

### 1. User/Auth Service

* registracija i prijava korisnika,

* autentifikacija i autorizacija putem JWT tokena,

* upravljanje korisničkim rolama (USER, ADMIN),

* CRUD operacije nad korisnicima.

* Baza: PostgreSQL

### 2. Football Data Service

* komunikacija sa eksternim sportskim API-jem,

* dobavljanje podataka o ligama, utakmicama i rezultatima,

* dobavljanje događaja (golovi, kartoni) i osnovne statistike,

* normalizacija i čuvanje podataka.

* Baza: PostgreSQL

* Keš: Redis (keširanje odgovora API-ja i live podataka)

### 3. Match Details & Statistics Service

* skladištenje događaja utakmica (timeline),

* čuvanje statistike mečeva kroz vreme,

* izlaganje API-ja za detalje utakmice.

* Baza: MongoDB

### 4. Analytics & Reporting Service

* obrada i agregacija sportskih podataka,

* generisanje analitika i statistika,

* priprema podataka za grafikone,

* generisanje PDF izveštaja o pojedinačnoj utakmici.

* Baza: PostgreSQL (agregirani podaci)

* Ulazni podaci: Football Data Service + Match Details Service

## Funkcionalnosti sistema
### Osnovne funkcionalnosti

* pregled dostupnih liga i takmičenja,

* pregled rasporeda i rezultata utakmica,

* prikaz detalja utakmice (rezultat, događaji, statistika),

* autentifikacija i autorizacija korisnika,

* CRUD operacije nad relevantnim entitetima.

### Personalizacija i praćenje sadržaja

S obzirom na to da sistem podržava autentifikovane korisnike, omogućena je
personalizacija sadržaja kroz funkcionalnost praćenja entiteta od interesa.

Autentifikovani korisnici mogu:

* da izdvoje i prate odabrane fudbalske lige i takmičenja,

* da prate konkretne utakmice koje ih interesuju,

* da na posebnoj stranici imaju pregled praćenih liga, utakmica i relevantnih
  događaja.

Ova funkcionalnost omogućava korisnicima brži pristup relevantnim informacijama
i predstavlja osnovu za buduća proširenja sistema, poput real-time notifikacija
u okviru diplomskog rada.

### Dodatne funkcionalnosti 1 (10 poena)

U okviru osnovnog projekta implementirane su:

* Generisanje PDF izveštaja

* Analitike u formi grafikona

### Dodatne funkcionalnosti 2 (10 poena)

U okviru osnovnog projekta implementirane su:

* Kombinacija relacionih i NoSQL baza podataka

* Keširanje podataka (Redis)

## Ograničenja i pristup live podacima

Zbog ograničenja besplatnih sportskih API-ja:

* koristi se periodični polling,

* primenjuje se keširanje podataka,

* arhitektura je dizajnirana tako da se lako može proširiti na real-time streaming u budućnosti (i/ili za potrebe diplomskog rada).

## Bodovanje projekta

Projekat je projektovan za ostvarivanje maksimalnih 80 poena:

* 60 poena – mikroservisna arhitektura (4 servisa, baze, CRUD, autentifikacija, komunikacija),

* 10 poena – dodatne funkcionalnosti (PDF + grafikoni),

* 10 poena – napredne funkcionalnosti (relacione + NoSQL baze, keširanje).

## Proširenja za diplomski rad

Projekat je dizajniran tako da može prerasti u diplomski rad kroz sledeća proširenja:

1. API Gateway

2. Asinhrona komunikacija između servisa

3. Notification Service (novi servis)

### Odgovornosti:

* praćenje događaja na utakmicama koje korisnik prati,

* slanje live notifikacija o:

  * postignutim golovima,

  * crvenim kartonima,

  * završetku utakmice.

Ovaj servis bi u diplomskoj verziji koristio asinhronu komunikaciju i omogućio real-time obaveštavanje korisnika.
