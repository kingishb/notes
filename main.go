package main

import (
	"flag"
	"fmt"
	"log"
	"os"
	"os/exec"
	"strings"
	"time"
)

func main() {

	home := os.Getenv("HOME")
	// notes files
	notes := fmt.Sprintf("%s/notes.txt", home)
	// occasional private notes to keep separate
	notesPersonal := fmt.Sprintf("%s/notes-personal.txt", home)

	// default editor is nvim
	editor := "nvim"

	// set up flags
	priv := flag.Bool("private", false, "")
	clear := flag.Bool("clear", false, "")
	code := flag.Bool("vscode", false, "")

	// short flags
	flag.BoolVar(priv, "p", false, "")
	flag.BoolVar(clear, "c", false, "")
	flag.BoolVar(code, "vs", false, "")
	flag.Usage = func() {
		fmt.Fprintf(flag.CommandLine.Output(), "Usage of %s:\n", "notes")
		fmt.Fprintf(flag.CommandLine.Output(), "  -p, -private\n    \tOpen non-work notes\n")
		fmt.Fprintf(flag.CommandLine.Output(), "  -c, -clear\n    \tInsert 50 newlines to not show notes on startup\n")
		fmt.Fprintf(flag.CommandLine.Output(), "  -vs, -vscode\n    \tUse VSCode\n")
	}
	flag.Parse()

	if *code {
		editor = "code"
	}

	var cmd *exec.Cmd
	if *priv {
		writeHeader(notesPersonal, *clear)
		cmd = exec.Command(editor, notesPersonal)
	} else {
		writeHeader(notes, *clear)
		cmd = exec.Command(editor, notes)
	}

	cmd.Stdin = os.Stdin
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	cmd.Run()
}

// writeHeader writes the date to the top of the notes file. clear
// puts a bunch of newlines to not show notes on a video call.
func writeHeader(file string, clear bool) {

	data, err := os.ReadFile(file)
	if err != nil {
		log.Fatal(err)
	}

	// build header in the Fomat
	// 01/02/2006
	// ----------
	now := time.Now()
	today := now.Format("01/02/2006")
	header := string(data)
	if strings.HasPrefix(header, today) {
		header = strings.Split(header, "\n")[0]
	} else {
		header = today
	}

	// if clear is set, add 50 newlines to hide current
	// buffer, for like, note taking on a video call
	n := strings.Repeat("\n", 50)
	if clear {

		// check if today's date is at top of file
		// if it is, dump 50 newlines after the date header
		if strings.HasPrefix(string(data), today) {
			err = os.WriteFile(file,
				[]byte(fmt.Sprintf("%s\n----------\n%s%s", header, n, string(data[len(header)+12:]))), 0644)
			if err != nil {
				log.Fatal(err)
			}
		// otherwise just write the header + lots of newlines
		} else {
			err = os.WriteFile(file,
				append([]byte(fmt.Sprintf("%s\n----------\n%s", header, n)),
					data...), 0644)
			if err != nil {
				log.Fatal(err)
			}
		}
	// otherwise just write today's date at the top
	// if it's not there.
	} else {
		if !strings.HasPrefix(string(data), today) {
			err = os.WriteFile(file,
				append([]byte(fmt.Sprintf("%s\n----------\n\n\n", header)),
					data...), 0644)
			if err != nil {
				log.Fatal(err)
			}
		}
	}
}
