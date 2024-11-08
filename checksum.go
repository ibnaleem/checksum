package main

import (
	"fmt"
	"os"
	"strings"
	"hash"
	"encoding/hex"
	"crypto/sha256"
	"crypto/md5"
	"crypto/sha1"
	"crypto/sha512"
)

var Reset = "\033[0m" 
var Red = "\033[31m" 
var Green = "\033[32m"
var Yellow = "\033[33m"

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func IdentifyHashType(hash string) string {
	if len(hash) == 32 {
		return "md5"
	} else if len(hash) == 40 {
		return "sha1"
	} else if len(hash) == 56 {
		return "sha224"
	} else if len(hash) == 64 {
		return "sha256"
	} else if len(hash) == 96 {
		return "sha384"
	} else if len(hash) == 128 {
		return "sha512"
	} else {
		return ""
	}
}

func CalculateFileHash(filePath string, hashType string) (string, error) {
	dat, err := os.ReadFile(filePath)
	check(err)

	var h hash.Hash

	switch hashType {
	case "md5":
		h = md5.New()
	case "sha1":
		h = sha1.New()
	case "sha224":
		h = sha256.New224()
	case "sha256":
		h = sha256.New()
	case "sha384":
		h = sha512.New384()
	case "sha512":
		h = sha512.New()
	default:
		return "", fmt.Errorf("unsupported hash type: %s", hashType)
	}

	h.Write(dat)
	hashBytes := h.Sum(nil)

	return hex.EncodeToString(hashBytes), nil
}

func main() {
	if len(os.Args) < 3 {
		fmt.Println(Red + "Usage: checksum <file_path> <hash>" + Reset)
		fmt.Println(Red + "Issues @ https://github.com/ibnaleem/checksum/issues" + Reset)
		os.Exit(1)
	} else {
		filePath := os.Args[1]
		hashValue := os.Args[2]
		hashType := IdentifyHashType(hashValue)
		
		if hashType == "" {
			fmt.Println(Red + "Unsupported hash type provided" + Reset)
			os.Exit(1)
		}
		
		fmt.Println(Yellow + "Using " + strings.ToUpper(hashType) + " algorithm to calculate hash for " + filePath + "..." + Reset)
		calculatedHash, err := CalculateFileHash(filePath, hashType)
		
		if err != nil {
			fmt.Printf("%s Error: %s %s", Red, err, Reset)
		} else {
			if calculatedHash == hashValue {
				fmt.Printf(Green + "Checksum matches for %s using %s algorithm!\n", filePath, strings.ToUpper(hashType))
				fmt.Println(Green + "Provided hash: " + hashValue)
				fmt.Println(Green + "Calculated hash: " + calculatedHash + Reset)
			} else {
				fmt.Printf(Red + "Checksum does not match for %s using %s algorithm!\n", filePath, hashType)
				fmt.Println(Red + "Provided hash: " + hashValue)
				fmt.Println(Red + "Calculated hash: " + calculatedHash)
			}
		}
	}
}
