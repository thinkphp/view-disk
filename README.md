# View-Disk: Disk Space Usage & Cleanup Tool

A fast and efficient Rust-based CLI tool to **view disk space usage** and **delete unwanted files or directories**.

## 🚀 Features
- 📊 **View Disk Space Usage**: Displays total and available disk space.
- 🗑️ **Delete Files & Directories**: Remove unwanted data securely.
- ⚡ **Fast & Lightweight**: Built with Rust for high performance.

## 📦 Installation

### 1️⃣ **Clone the Repository**
```sh
git clone https://github.com/your-username/view-disk.git
cd view-disk
```

### 2️⃣ **Build the Project**
```sh
cargo build --release
```

### 3️⃣ **Run the Executable**
```sh
./target/release/view-disk --help
```

---

## 🛠️ Usage

### ✅ **View Disk Space Usage**
```sh
cargo run -- --view
```
or, if built:
```sh
./target/release/view-disk --view
```

### 🗑️ **Delete a File**
```sh
cargo run -- --delete /path/to/file
```

### 📂 **Delete a Directory**
```sh
cargo run -- --delete /path/to/directory
```

---

## 📝 Example Output
```
Disk Usage:
Name: "/dev/sda1", Total: 500000 MB, Available: 320000 MB
```
or when deleting:
```
File "/home/user/old.log" deleted successfully.
```

---

## 🔧 Development

### **Run in Debug Mode**
```sh
cargo run -- --view
```

### **Format Code**
```sh
cargo fmt
```

### **Run Clippy for Linting**
```sh
cargo clippy
```

---

## 📜 License
MIT License © 2025 Your Name

---

## 🤝 Contributing
Pull requests are welcome! Feel free to open an issue for feature requests or bug reports.


