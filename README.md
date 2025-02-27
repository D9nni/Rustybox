# Rustybox

Rustybox ofera o implementare in Rust a catorva comenzi elementare din Linux, folosind apeluri de functii de sistem.


Describe your solution for the homework.

Solutia temei nu e unitara si nu respecta acelasi pattern peste tot, codul nu arata foarte bine, dar se poate
observa un progres de la inceput pana la sfarsit.

Functia invalid() da exit(-1) si afiseaza Invalid command.

1. Primele functii (pwd, echo, cat) sunt foarte simple si primesc ca argument vectorul args prin valoare.
2. mkdir primeste ca argument un slice cu argumentele specifice (cum ar fi trebuit la toate) si returneaza un cod
de eroare deoarece e apelat intern si din alte functii
3. mv foloseste fs::rename (initial o gandisem ca pe un copy, adica sa creeze manual alt fisier si sa scrie in el)
4. ln verifica corectitudinea argumentelor, apoi face un hard sau symbolic link
5. rmdir foloseste fs::remove_dir
6. rm verifica daca exista oricat de multe argumente -r sau -d ori vreunul invalid, apoi trateaza cazurile
de stergere in functie de argumentele primite. Nu a fost necesara o recursivitate -r pentru ca se foloseste functia
remove_dir_all 
7. ls este ajutat de functiile ls_elementar (pentru cazul de baza) si ls_rec (pentru -R). El se ocupa de argumentele primite si apoi
apeleaza functia ajutatoare de care are nevoie. ls_elementary afiseaza si returneaza continutul sortat al unui director sau numele fisierului primit, iar ls_rec foloseste ls_elementary pentru a printa continutul fiecarui director, apoi continua recursivitatea in cazul in care in output
au existat directoare. 
8. cp primeste argumentele si trateaza toate cazurile. Pentru -r apeleaza functia ajutatoare cp_helper, asemanatoare. Variabila errno returneaza codul de eroare, dar e folosita si ca flag pentru cazul in care si sursa si destinatia sunt directoare. Au fost folosite si unwrap-uri pentru operatiile care sunt "sigure" pentru a scurta codul. La cazul de baza se ajunge la a face un fs::copy. In comentarii au ramas niste urme de la println-urile folosite pentru erori care pot ajuta la intelegerea lui.
9. touch parseaza argumentele si seteaza pe true niste variabile bool. Daca fisierul specificat nu exista, il creeaza. Daca exista, il deschide si citeste sau scrie din el pentru a modifica data ultimului acces/modificari. Nu am gasit o posibilitate de a seta last modified fara last accessed (cazul if !a&&m)
10. chmod verifica daca permisiunile sunt date ca numar, in acest caz facand conversia la un numar in baza 8 cu from_str_radix, iar in cazul nefericit in care sunt date ca String merge pana la + si verifica daca exista doar u/g/o/a, apoi dupa + daca exista doar r/w/x altfel returneaza invalid. Pe baza acestor informatii, folosind vectorii person si mode construieste numarul construit special ca in baza 8 mode_nr. Daca operatia era de adaugat permisiuni face or pe biti cu permisiunile avute deja, daca era de eliminat permisiuni face si cu inversul lui mode_nr.

## Verify

Run the following commands to test your homework:

You will have to install NodeJS (it is installed in the codespace)

```bash
# Clone tests repository
git submodule update --init 

# Update tests repository to the lastest version
cd tests
git pull 
cd ..

# Install loadash
npm install lodash
```

Install rustybox

```bash
cargo install --path .
```

Run tests

```bash
cd tests
# Run all tests 
./run_all.sh

# Run single test
./run_all.sh pwd/pwd.sh
```
