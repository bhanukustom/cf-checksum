package main

import (
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"io"
	"io/fs"
	"log"
	"net/http"
	"os"
	"path/filepath"
)

// dirChecksum hashes relative path + contents of every file; WalkDir is lexical so it's deterministic.
func dirChecksum(root string) (string, int, error) {
	h := sha256.New()
	files := 0
	err := filepath.WalkDir(root, func(p string, d fs.DirEntry, err error) error {
		if err != nil || d.IsDir() {
			return err
		}
		rel, _ := filepath.Rel(root, p)
		io.WriteString(h, rel+"\x00")
		f, err := os.Open(p)
		if err != nil {
			return err
		}
		defer f.Close()
		files++
		_, err = io.Copy(h, f)
		return err
	})
	return hex.EncodeToString(h.Sum(nil)), files, err
}

func main() {
	dir := os.Getenv("CHECKSUM_DIR")
	if dir == "" {
		dir = "/data"
	}
	http.HandleFunc("/health", func(w http.ResponseWriter, _ *http.Request) { w.Write([]byte("ok")) })
	http.HandleFunc("/checksum", func(w http.ResponseWriter, _ *http.Request) {
		sum, n, err := dirChecksum(dir)
		if err != nil {
			http.Error(w, err.Error(), 500)
			return
		}
		json.NewEncoder(w).Encode(map[string]any{"dir": dir, "algo": "sha256", "files": n, "checksum": sum})
	})
	log.Fatal(http.ListenAndServe(":4000", nil))
}
