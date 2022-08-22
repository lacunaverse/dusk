package main

import (
	"database/sql"
	_ "embed"
	"encoding/json"
	"errors"
	"html/template"
	"io"
	"log"
	"math/rand"
	"net/http"
	"net/url"
	"time"

	"github.com/gorilla/mux"
	_ "github.com/mattn/go-sqlite3"
)

type Templates struct {
	index  *template.Template
	errors *template.Template
}

func (t *Templates) Render(w io.Writer, name string, data interface{}, cat string) error {
	switch cat {
	case "index":
		return t.index.ExecuteTemplate(w, name, data)
	case "errors":
		return t.errors.ExecuteTemplate(w, name, data)
	default:
		return t.errors.ExecuteTemplate(w, name, data)
	}
}

type NotFound struct {
}

func (n NotFound) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	t.Render(w, "404.html", "", "errors")
}

var t = &Templates{
	index:  template.Must(template.ParseFiles("views/layout.html", "views/index.html", "views/layouts/nav.html")),
	errors: template.Must(template.ParseFiles("views/layout.html", "views/errors/404.html", "views/layouts/nav.html")),
}

// Index root route
func Index(w http.ResponseWriter, r *http.Request) {
	t.Render(w, "index.html", "", "index")
}

type Error struct {
	Error string `json:"error"`
}

func sendError(w http.ResponseWriter, err string) {
	w.WriteHeader(http.StatusBadRequest)
	w.Header().Add("Content-Type", "application/json")
	encoder := json.NewEncoder(w)
	encoder.Encode(&Error{Error: err})
}

type AddLinkRequest struct {
	Link     string `json:"link"`
	Stopcode string `json:"stopcode"`
}

type AddLinkItem struct {
	Link     *url.URL
	Stopcode string
	ID       string
}

func AddLink(w http.ResponseWriter, r *http.Request) {
	linkRequest := &AddLinkRequest{}
	decoder := json.NewDecoder(r.Body)

	err := decoder.Decode(linkRequest)
	if err != nil {
		sendError(w, "Invalid response.")
	}

	item := &AddLinkItem{}
	u, err := url.Parse(linkRequest.Link)
	if err != nil {
		sendError(w, "Invalid URL.")
	}

	item.Link = u
	item.Stopcode = linkRequest.Stopcode

	unique := false
	for !unique {
		item.ID = generateID()
		_, err := db.get_id(item.ID)
		if err.Error() == "not found" {
			unique = true
			break
		}
	}

	err = db.add_link(item)
	if err != nil {
		sendError(w, "Failed to save shortened link.")
		return
	}

	w.WriteHeader(http.StatusCreated)
	w.Header().Add("Content-Type", "application/json")
	encoder := json.NewEncoder(w)
	encoder.Encode(&struct {
		ID string `json:"id"`
	}{item.ID})
}

var chars = []string{
	"1", "2", "3", "4", "5", "6", "7", "8", "9", "0",
	"a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z",
	"A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z",
}

func generateID() string {
	id := ""
	for i := 0; i < 10; i++ {
		rand.Seed(time.Now().UnixNano())

		id += chars[rand.Intn(len(chars))]
	}
	return id
}

func Redirect(w http.ResponseWriter, r *http.Request) {
	link, err := db.get_id(mux.Vars(r)["id"])

	if err != nil {
		t.Render(w, "errors", nil, "404.html")
	}

	w.Header().Add("Location", link.Link.String())
	w.WriteHeader(http.StatusPermanentRedirect)
}

func main() {
	database, err := sql.Open("sqlite3", "./store/main")
	if err != nil {
		log.Fatal(err)
	}
	defer database.Close()

	db = &Database{database}
	err = db.initialize()
	if err != nil {
		log.Fatal(err)
	}

	r := mux.NewRouter()

	r.PathPrefix("/static/").Handler(http.StripPrefix("/static/", http.FileServer(http.Dir("./static/dist")))).Methods("GET")
	r.HandleFunc("/", Index).Methods("GET")
	r.HandleFunc("/add", AddLink).Methods("POST")
	r.HandleFunc("/l/{id}", Redirect).Methods("GET")

	r.NotFoundHandler = NotFound{}
	log.Fatal(http.ListenAndServe(":8000", r))

}

type Database struct {
	*sql.DB
}

//go:embed sql/init.sql
var QUERY_INIT string

func (db *Database) initialize() error {
	_, err := db.Exec(QUERY_INIT)

	return err
}

func (db *Database) add_link(req *AddLinkItem) error {
	tx, err := db.Begin()
	if err != nil {
		return err
	}

	statement, err := tx.Prepare("insert into links values (?, ?, ?)")
	if err != nil {
		return err
	}

	defer statement.Close()

	_, err = statement.Exec(req.ID, req.Link.String(), req.Stopcode)
	if err != nil {
		return err
	}

	return tx.Commit()
}

