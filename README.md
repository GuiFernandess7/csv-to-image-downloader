# 🦀 CSV to JSON Converter and Image Downloader

Convert CSV data to JSON and download images from URL links.

![Untitled Diagram drawio](https://github.com/user-attachments/assets/7962063c-703f-4f52-94b4-d415ffcef368)

# **How to Use**

### **Description**

This tool converts CSV files containing image URLs to JSON format and downloads the images to a local folder. It's a simple way to process image data and automate the image downloading process from a CSV.

### **Prerequisites**

Before getting started, ensure you have the following installed on your machine:

1. **Git** – For cloning the repository.
2. **Rust** – To compile and run the program.
3. **Cargo** – Rust's package manager to handle dependencies and run the project.

You can install Git and Rust by following the instructions on their official websites:

- [Install Git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git)
- [Install Rust](https://www.rust-lang.org/tools/install)

### **Step 1: Clone the Repository**

To get started, clone the repository to your local machine. Open your terminal and run the following command:

```bash
git clone https://github.com/GuiFernandess7/csv-to-image-downloader.git
```

This will download the project files to a folder named `csv-to-image-downloader`.

### **Step 2: Navigate to the Project Directory**

Once the repository is cloned, change to the project directory:

```bash
cd csv-to-image-downloader
```

### **Step 3: Build the Project**

Now that you're in the project folder, you need to build the project with `cargo`, which handles the Rust compilation process:

```bash
cargo build
```

This will compile the project and download the necessary dependencies.

### **Step 4: Run the Project**

The project is designed to work with two modes: **convert** (to convert a CSV file to JSON) and **download** (to download images from URLs listed in a CSV file).

To run the project, use the following syntax:

```bash
cargo run -- <mode> <arguments>
```

#### Mode 1: Convert CSV to JSON

To convert a CSV file into JSON format, use the `convert` mode. For example:

```bash
cargo run -- convert --file-path path/to/your/file.csv path/to/your/output.json
```

This will read the CSV file at `path/to/your/file.csv`, convert it to JSON format, and save it as `images.json` (or any other output path you specify in the code).

#### Mode 2: Download Images from CSV URLs

If you want to download images from URLs listed in a CSV file, use the `download` mode. For example:

```bash
cargo run -- download --file-data-path path/to/your/file.csv --output-folder path/to/output/folder
```

This will:

1. Read the CSV file at `path/to/your/file.csv` to extract image URLs.
2. Download all images listed in the CSV and save them to the specified `output-folder`.

### **Step 5: Example CSV Format**

For the tool to work properly, your CSV file should have a structure like this:

```csv
id,name,url
1,Image 1,https://example.com/image1.jpg
2,Image 2,https://example.com/image2.jpg
```

- The **id** column is for any identifier.
- The **name** column is for the image's name or description.
- The **url** column should contain the full URL of the image to be downloaded.

### **Step 6: Output Files**

- **JSON Output**: When using the `convert` mode, the output will be a JSON file that contains the data from your CSV file in JSON format. Example:

```json
[
  {
    "id": 1,
    "name": "Image 1",
    "url": "https://example.com/image1.jpg"
  },
  {
    "id": 2,
    "name": "Image 2",
    "url": "https://example.com/image2.jpg"
  }
]
```

- **Downloaded Images**: When using the `download` mode, the images from the URLs in the CSV will be saved to the specified output folder.

### **Step 7: Troubleshooting**

If you encounter any issues while running the project, here are a few things to check:

- **Ensure that your CSV file is correctly formatted**: The program expects a valid CSV with columns for `id`, `name`, and `url`.
- **Check the image URLs**: Ensure the image URLs are valid and publicly accessible.
- **Permissions**: Make sure you have write permissions in the output folder where the images are being downloaded.
- **Dependencies**: If there are any issues with dependencies, try running `cargo update` to update them.

### **Step 8: Contributing**

If you'd like to contribute to this project, feel free to fork the repository and create a pull request with your changes. Whether it's fixing a bug, adding a feature, or improving documentation, your contribution is welcome!

---

### **Final Notes**

This tool is designed to be simple and efficient for processing CSV files with image URLs. It saves you time by automating the image download process and converting data into JSON format for easier handling.

If you have any questions or encounter issues, don't hesitate to open an issue on GitHub or submit a pull request!

---

### **Summary**

1. **Clone** the repository: `git clone https://github.com/GuiFernandess7/csv-to-image-downloader.git`
2. **Navigate** to the project folder: `cd csv-to-image-downloader`
3. **Build** the project: `cargo build`
4. **Run** the project using one of the modes:
   - Convert: `cargo run -- convert --file-path path/to/your/file.csv`
   - Download: `cargo run -- download --file-data-path path/to/your/file.csv --output-folder path/to/output/folder`
5. **Troubleshoot** any issues by checking permissions, CSV formatting, or dependencies.
