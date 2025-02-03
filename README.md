<h1 align="center" id="title">🗿​GitGaChadSaver🗿​</h1>

<p align="center"><img src="https://socialify.git.ci/Dornagol/GitGaChadSaver/image?font=Source+Code+Pro&amp;language=1&amp;name=1&amp;owner=1&amp;pattern=Floating+Cogs&amp;pulls=1&amp;stargazers=1&amp;theme=Auto" alt="project-image"></p>

<p align="center" id="description">Le bot qui fait tes push Git pendant que tu bourlingue autre chose que ton clavier 😎​</p>
<p align="center">(Encore en phase de prép, ça arrive fort👴)</p>

## 🏋️🗿​ Pourquoi utiliser GitGaChadSaver ?
- 🚀 Push automatique parce que tu sais pas faire
- 💪 Occupe toi de te bumble bee le megatron au lieu de push
- 😎 Zéro effort, 100% swag

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
   - 🔥 Nom : `"GitGaChad MVP"`
   - ⚡ Déclencheur : `À la fermeture de session`
   - 🦾 Action : Lancer `gitgachadsaver.exe`

---

## 🚨 Avertissements
> ❌ **Ne pas utiliser si :**
> - Tu préfères `git push` comme un peasant
> - T'as peur de devenir trop puissant
> - Tu met des air force one en big 2025

---

## 🤝 Rejoins la Brotherhood
1. Lance `cargo fmt` avant de commit (hygiène de Chad)
2. Teste sur **Linux** ET **Windows** (vrai Chad = cross-platform)
3. Fais des PR plus balèzes que tes biceps

---

📜 **License** : [MIT](LICENSE) *(Traduction : "Fais-en ce que tu veux, frérot")*
