<h1 align="center" id="title">🗿​GitGaChadSaver🗿​</h1>

<p align="center"><img src="https://socialify.git.ci/Dornagol/GitGaChadSaver/image?font=Source+Code+Pro&amp;language=1&amp;name=1&amp;owner=1&amp;pattern=Floating+Cogs&amp;pulls=1&amp;stargazers=1&amp;theme=Auto" alt="project-image"></p>

<p align="center" id="description">Le bot qui fait tes push Git pendant que tu bourlingue autre chose que ton clavier 😎​</p>
<p align="center">(Encore en phase de prép, ça arrive fort👴)</p>

```bash
# Installation Chad Style
git clone https://github.com/Dornagol/GitGaChadSaver
cd GitGaChadSaver && cargo build --release
```

---

## 🔌 Config

### 🐧 Linux
```bash
# Crée le fichier de service
echo "[Unit]
Description=GitGaChadSaver - Push comme un vrai
Before=shutdown.target

[Service]
Type=oneshot
ExecStart=$(pwd)/target/release/gitgachadsaver
WorkingDirectory=$(pwd)

[Install]
WantedBy=shutdown.target" | sudo tee /etc/systemd/system/gitgachad.service

# Active les gains
sudo systemctl enable gitgachad.service
```

### 🪟 Windows
1. Ouvre le **Planificateur de tâches**
2. Crée une tâche :
   - Nom : `"GitGaChad MVP"`
   - Déclencheur : `À la fermeture de session`
   - Action : Lancer `gitgachadsaver.exe`
---

📜 **License** : [MIT](LICENSE) *(Traduction : "Fais-en ce que tu veux frérot")*
