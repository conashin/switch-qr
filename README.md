# Server rMQR Code Generator

這個命令列工具可以將包含伺服器資訊與 Google Drive 連結的 YAML 檔案，轉換為高度最佳化的 rMQR Code 圖片，非常適合列印成小尺寸貼紙貼在機器上。

## 為什麼使用 rMQR？

為了解決傳統 QR Code 在窄長空間下難以適應的問題，本工具改採用 **rMQR (Rectangular Micro QR Code)** 技術：

1.  **長條形狀**：rMQR 天生就是設計為長條狀，非常適合用來儲存單一網址（如 Google Drive 檔案連結），同時又能完美貼合在伺服器邊框或是各種線材標籤等狹窄平面上。
2.  **無留白邊緣 (No Quiet Zone)**：本工具刻意將白邊移除（`quiet_zone = 0.0`），最大化每一顆黑白方塊的物理面積。
3.  **精簡資料**：為了保持最細小的面積，我們捨棄了將所有伺服器屬性轉為 JSON 塞入條碼的做法。條碼內唯一儲存的內容就是「您的 Google Drive 連結」，直接把所有伺服器資訊以檔案的形式放在雲端，掃描後一鍵存取！

## 使用教學

### 1. 準備 YAML 檔案

建立一個包含伺服器名稱與網址的 YAML 檔（例如 `input.yaml`）：

```yaml
name: "Server01"
url: "https://drive.google.com/file/d/xxxxxxx/view"
```

### 2. 執行程式產生圖片

如果您已經下載了編譯好的執行檔 (例如名為 `app`)，請在命令列中執行：

```bash
./app input.yaml
```

執行成功後，您會在當前目錄下看到一張名為 `Server01.png` 的圖片檔，這就是您的優化版 rMQR Code。

## GitHub Actions 自動發布

本專案配置了 GitHub Actions CI/CD 流程：
當您推送(Push)一個符合 `v*` 命名規則的標籤 (Tag，例如 `v1.0.0`) 時，系統會自動在雲端編譯出以下四個平台的執行檔，並建立一個 GitHub Release：

*   Windows x86_64
*   Windows arm64
*   Linux x86_64
*   Linux arm64

您可以直接到 Releases 頁面下載符合您系統架構的執行檔。
