import re, sys, hashlib
from rich.console import Console

def identify_hash(s: str) -> str:
    """
    Identifies the hash type of a given string (s) based on its length
    and character set

    Args:
    s (str): a hash value of type string

    Returns:
    str: the hash type of s
    """
    if len(s) == 32 and re.match(r'^[a-fA-F0-9]{32}$', s):
        return "md5"
    elif len(s) == 40 and re.match(r'^[a-fA-F0-9]{40}$', s):
        return "sha1"
    elif len(s) == 56 and re.match(r'^[a-fA-F0-9]{56}$', s):
        return "sha224"
    elif len(s) == 64 and re.match(r'^[a-fA-F0-9]{64}$', s):
        return "sha256"
    elif len(s) == 96 and re.match(r'^[a-fA-F0-9]{96}$', s):
        return "sha384"
    elif len(s) == 128 and re.match(r'^[a-fA-F0-9]{128}$', s):
        return "sha512"
    else:
        return ""

def calculate_hash(file_path: str, hash_type: str) -> str:
    """
    Calculates the hash for a file.

    Args:
    file_path (str): Path to the file.
    hash_type (str): The type of hash to calculate for the file.

    Returns:
    str: hash of the file.
    """

    if hash_type == "md5":
        file_hash = hashlib.md5()
    elif hash_type == "sha1":
        file_hash = hashlib.sha1()
    elif hash_type == "sha224":
        file_hash = hashlib.sha224()
    elif hash_type == "sha256":
        file_hash = hashlib.sha256()
    elif hash_type == "sha384":
        file_hash = hashlib.sha384()
    elif hash_type == "sha512":
        file_hash = hashlib.sha512()
        
    with open(file_path, "rb") as f:
        chunk = 0
        while chunk := f.read(4096):
            file_hash.update(chunk)
    return file_hash.hexdigest()

def main():
    console = Console()
    if (len(sys.argv) -1) > 2:
        console.print("[bold red]Too many args\nUsage: python3 checksum.py /path/to/file hash_value[/bold red]")
    elif (len(sys.argv) -1) < 2:
        console.print("[bold red]Too few args\nUsage: python3 checksum.py /path/to/file hash_value[/bold red]")
    else:
        file_path = sys.argv[1]
        hash_value = sys.argv[2]
        try:
            hash_type = identify_hash(hash_value)
            console.print(f"[bold green]Using {hash_type.upper()} Algorithm...[/bold green]")
            checksum = calculate_hash(file_path, hash_type)
            if checksum == hash_value:
                out = f"[bold green]File hash is authentic\nProvided hash: {hash_value}\nCalculated hash: {checksum}[/bold green]"
            else:
                out = f"[bold red]File hash IS NOT authentic\nProvided hash: {hash_value}\nCalculated hash: {checksum}[/bold red]"
            console.print(out)
        except FileNotFoundError:
            print("File not found!")
        except IsADirectoryError:
            print("Given path is a directory, please provide a file path.")
        except Exception as e:
            print(f"An error occurred: {e}")
    
if __name__ == "__main__":
    main()
