import os
import shutil
import sys

try:
    target_directory_for_bin = sys.argv[1]
except IndexError:
    print("No target directory for binary given as a command line argument #1.", file=sys.stderr)
    sys.exit(1)

# Get project directory, which is the parent directory
project_dir = os.path.dirname(__file__)
project_dir = os.path.join(project_dir, "..")
print("Project directory", project_dir)

os.chdir(project_dir)

# Build Bible parse binary
os.system("cargo build --release")

bin_name = "bible_ref_parse"

# Copy the binary to the bin directory
shutil.copyfile(f"{project_dir}/target/release/{bin_name}",
                f"{target_directory_for_bin}/{bin_name}")

# Set execution permissions to the binary
os.system(f"chmod +x {target_directory_for_bin}/{bin_name}")
