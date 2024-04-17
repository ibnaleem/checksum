import hashlib

def generate_sha256(file_path) -> str:
    """
    Generate SHA256 hash for a file.

    Args:
    file_path (str): Path to the file.

    Returns:
    str: SHA256 hash of the file.
    """
    sha256_hash = hashlib.sha256()
    with open(file_path, "rb") as f:
        chunk = 0
        while chunk := f.read(4096):
            sha256_hash.update(chunk)
    return sha256_hash.hexdigest()

if __name__ == "__main__":

    file_path = input("Enter the path of the file: ")
    file_path = file_path.strip('"')
    checksum = input("Enter checksum: ")
    try:
        sha256 = generate_sha256(file_path)
        print(sha256 == checksum)
    except FileNotFoundError:
        print("File not found!")
    except IsADirectoryError:
        print("Given path is a directory, please provide a file path.")
    except Exception as e:
        print(f"An error occurred: {e}")
